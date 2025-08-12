'use client';

import { useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { useAuth } from '@/context/auth-context';

/**
 * @param {string[]} allowedRoles - Array of allowed roles for a page
 */
export default function useRoleGuard(allowedRoles = []) {
  const { user, loading } = useAuth();
  const router = useRouter();

  useEffect(() => {
    if (loading) return; // Wait for auth to load

    if (!user) {
      router.push('/login'); // Not logged in
    } else if (!allowedRoles.includes(user.role)) {
      router.push('/unauthorized'); // Logged in but wrong role
    }
  }, [user, loading, router, allowedRoles]);
}
