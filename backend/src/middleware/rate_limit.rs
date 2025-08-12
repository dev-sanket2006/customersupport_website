use axum::{
    body::Body,
    extract::ConnectInfo,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use governor::{Quota, RateLimiter};
use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr},
    num::NonZeroU32,
    sync::{Arc, Mutex},
    time::Duration,
};

// Simple in-memory rate limiter using governor directly
type SharedRateLimiter = Arc<Mutex<HashMap<IpAddr, Arc<RateLimiter<governor::state::direct::NotKeyed, governor::state::InMemoryState, governor::clock::DefaultClock>>>>>;

/// Create a rate limiter instance
pub fn create_rate_limiter() -> SharedRateLimiter {
    Arc::new(Mutex::new(HashMap::new()))
}

/// Rate limiting middleware function
pub async fn rate_limit_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract IP address
    let ip = get_real_ip(&request, addr.ip());
    
    // Create or get rate limiter for this IP
    let quota = Quota::per_second(NonZeroU32::new(1).unwrap())
        .allow_burst(NonZeroU32::new(30).unwrap());
    
    let limiter = Arc::new(RateLimiter::direct(quota));
    
    // Check if request is allowed
    match limiter.check() {
        Ok(_) => {
            // Request allowed, continue
            Ok(next.run(request).await)
        }
        Err(_) => {
            // Rate limit exceeded
            Err(StatusCode::TOO_MANY_REQUESTS)
        }
    }
}

/// Extract real IP from headers (for reverse proxy support)
fn get_real_ip(request: &Request<Body>, fallback_ip: IpAddr) -> IpAddr {
    // Check common proxy headers in order of preference
    let headers = request.headers();
    
    // X-Forwarded-For (most common)
    if let Some(xff) = headers.get("x-forwarded-for") {
        if let Ok(xff_str) = xff.to_str() {
            if let Some(first_ip) = xff_str.split(',').next() {
                if let Ok(ip) = first_ip.trim().parse::<IpAddr>() {
                    return ip;
                }
            }
        }
    }
    
    // X-Real-IP
    if let Some(real_ip) = headers.get("x-real-ip") {
        if let Ok(ip_str) = real_ip.to_str() {
            if let Ok(ip) = ip_str.parse::<IpAddr>() {
                return ip;
            }
        }
    }
    
    // CF-Connecting-IP (Cloudflare)
    if let Some(cf_ip) = headers.get("cf-connecting-ip") {
        if let Ok(ip_str) = cf_ip.to_str() {
            if let Ok(ip) = ip_str.parse::<IpAddr>() {
                return ip;
            }
        }
    }
    
    // Fallback to connection IP
    fallback_ip
}