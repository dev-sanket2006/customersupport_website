"use client";

import { useEffect, useRef, useState } from "react";
import { useParams } from "next/navigation";
import { useAuth } from "@/context/auth-context";

export default function UserTicketDetail() {
  const { id } = useParams();
  const { token, user } = useAuth();
  const [ticket, setTicket] = useState(null);
  const [messages, setMessages] = useState([]);
  const [newMessage, setNewMessage] = useState("");
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [wsConnected, setWsConnected] = useState(false);

  const socketRef = useRef(null);
  const bottomRef = useRef(null);

  // Fetch ticket + messages
  useEffect(() => {
    if (!id || !token) return;

    const fetchData = async () => {
      try {
        const [ticketRes, messagesRes] = await Promise.all([
          fetch(`http://localhost:8000/tickets/${id}`, {
            headers: { Authorization: `Bearer ${token}` },
          }),
          fetch(`http://localhost:8000/messages/${id}`, {
            headers: { Authorization: `Bearer ${token}` },
          }),
        ]);

        if (!ticketRes.ok) throw new Error(await ticketRes.text());
        if (!messagesRes.ok) throw new Error(await messagesRes.text());

        const ticketData = await ticketRes.json();
        const messagesData = await messagesRes.json();

        setTicket(ticketData);
        setMessages(messagesData);
      } catch (err) {
        setError(err.message || "Failed to load data");
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, [id, token]);

  // WebSocket setup with detailed logging
  useEffect(() => {
    console.log("🔍 WebSocket useEffect triggered");
    console.log("📊 Current state:", {
      id: !!id,
      token: !!token,
      user: !!user,
      id_value: id,
      token_length: token?.length,
      user_id: user?.id,
    });

    if (!id || !token || !user) {
      console.log("❌ Missing required data - not connecting WebSocket:", {
        id: !!id,
        token: !!token,
        user: !!user,
      });
      return;
    }

    const wsUrl = `ws://localhost:8000/ws/tickets/${id}?token=${encodeURIComponent(
      token
    )}`;
    console.log("🌐 Creating WebSocket connection to:", wsUrl);
    console.log("🔑 Token (first 50 chars):", token.substring(0, 50) + "...");
    console.log("👤 User ID:", user.id);

    const ws = new WebSocket(wsUrl);

    ws.onopen = (event) => {
      console.log("✅ WebSocket OPENED successfully!");
      console.log("🔗 Connection event:", event);
      console.log("📡 WebSocket readyState:", ws.readyState);
      console.log("🌐 WebSocket URL:", ws.url);
      setWsConnected(true);
    };

    ws.onmessage = (event) => {
      console.log("📥 WebSocket message received:", event.data);
      try {
        const newMsg = JSON.parse(event.data);
        console.log("✅ Parsed message:", newMsg);
        setMessages((prev) => {
          const exists = prev.some((msg) => msg.id === newMsg.id);
          if (exists) {
            console.log("⚠️ Message already exists, not adding:", newMsg.id);
          } else {
            console.log("➕ Adding new message:", newMsg.id);
          }
          return exists ? prev : [...prev, newMsg];
        });
      } catch (err) {
        console.error("❌ Failed to parse WebSocket message:", err);
        console.error("📄 Raw message data:", event.data);
      }
    };

    ws.onerror = (err) => {
      console.error("❌ WebSocket ERROR occurred:");
      console.error("🔍 Error event:", err);
      console.error("📡 WebSocket readyState:", ws.readyState);
      console.error("🌐 WebSocket URL:", ws.url);
      console.error("🔧 WebSocket constructor:", ws.constructor.name);
    };

    ws.onclose = (e) => {
      console.warn("🔌 WebSocket CLOSED:");
      console.warn(
        `📊 Close details: code=${e.code}, reason='${e.reason || "No reason"}'`
      );
      console.warn("🔍 Close event object:", e);
      console.warn("📡 Final readyState:", ws.readyState);

      // Common WebSocket close codes explanation
      const closeReasons = {
        1000: "Normal closure",
        1001: "Going away",
        1002: "Protocol error",
        1003: "Unsupported data",
        1006: "Abnormal closure (usually network/auth issues)",
        1007: "Invalid frame payload data",
        1008: "Policy violation",
        1009: "Message too big",
        1010: "Missing extension",
        1011: "Internal error",
        1015: "TLS handshake failure",
      };

      console.warn(
        `💡 Close code meaning: ${closeReasons[e.code] || "Unknown reason"}`
      );
      setWsConnected(false);
    };

    console.log("🔗 WebSocket object created, setting up ref...");
    socketRef.current = ws;

    return () => {
      console.log("🧹 Cleaning up WebSocket connection");
      console.log("📡 WebSocket state before cleanup:", ws.readyState);
      if (
        ws.readyState === WebSocket.OPEN ||
        ws.readyState === WebSocket.CONNECTING
      ) {
        console.log("🔌 Closing WebSocket connection");
        ws.close();
      }
    };
  }, [id, token, user]);

  // Scroll to bottom when new message arrives
  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  const handleSendMessage = async () => {
    if (!newMessage.trim()) return;

    const payload = {
      ticket_id: id,
      sender_id: user?.id,
      content: newMessage,
      is_from_customer: true, // ✅ important for sender role
      channel: "web",
    };

    try {
      const res = await fetch("http://localhost:8000/messages", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${token}`,
        },
        body: JSON.stringify(payload),
      });

      if (!res.ok) throw new Error(await res.text());

      // ✅ Don't send via WebSocket or update UI manually
      setNewMessage(""); // just clear input
    } catch (err) {
      console.error("❌ Message send error:", err);
    }
  };

  if (loading) return <p>Loading...</p>;
  if (error) return <p className="text-red-500">{error}</p>;
  if (!ticket) return <p>No ticket found.</p>;

  return (
    <div className="p-6 max-w-2xl mx-auto bg-white rounded-lg shadow text-black">
      <h1 className="text-2xl font-bold mb-4">🎫 Ticket #{ticket.id}</h1>

      <p className="mb-2">
        <strong>Subject:</strong> {ticket.subject}
      </p>
      <p className="mb-2">
        <strong>Description:</strong> {ticket.description}
      </p>
      <p className="mb-2">
        <strong>Status:</strong> {ticket.status}
      </p>
      <p className="mb-6">
        <strong>Created At:</strong>{" "}
        {new Date(ticket.created_at).toLocaleString()}
      </p>

      <div className="mt-6">
        <h2 className="text-xl font-semibold mb-2">Messages</h2>
        <div className="max-h-64 overflow-y-auto space-y-3 mb-3 border p-2 rounded">
          {messages.map((msg) => {
            // Use is_from_customer flag instead of sender_id comparison
            const isFromCustomer = msg.is_from_customer;
            const isMe = isFromCustomer; // For user page, "me" means customer messages
            const displayName = isFromCustomer
              ? "You"
              : msg.external_sender_email || "Support Agent";
            const createdAt = msg.created_at
              ? new Date(msg.created_at).toLocaleString()
              : "unknown time";

            return (
              <div
                key={msg.id}
                className={`p-2 rounded text-sm ${
                  isMe ? "bg-blue-100 text-right" : "bg-gray-100 text-left"
                }`}
              >
                <p>{msg.content}</p>
                <p className="text-xs text-gray-500 mt-1">
                  {displayName} via {msg.channel || "web"}, {createdAt}
                </p>
              </div>
            );
          })}
          <div ref={bottomRef} />
        </div>

        <textarea
          rows={2}
          className="w-full border rounded p-2 mb-2"
          placeholder="Type your message..."
          value={newMessage}
          onChange={(e) => setNewMessage(e.target.value)}
        />
        <button
          className="px-4 py-1 bg-blue-600 text-white rounded hover:bg-blue-700 disabled:opacity-50"
          onClick={handleSendMessage}
          disabled={!newMessage.trim()}
        >
          Send
        </button>

        <p className="mt-2 text-sm text-gray-500">
          WebSocket:{" "}
          <span className={wsConnected ? "text-green-600" : "text-red-600"}>
            {wsConnected ? "Connected" : "Disconnected"}
          </span>
        </p>
      </div>
    </div>
  );
}
