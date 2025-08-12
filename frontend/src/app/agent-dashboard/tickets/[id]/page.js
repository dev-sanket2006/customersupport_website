"use client";

import { useEffect, useRef, useState } from "react";
import { useParams } from "next/navigation";
import { useAuth } from "@/context/auth-context";

export default function AgentTicketDetail() {
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

  useEffect(() => {
    if (!id || !token) return;

    const fetchData = async () => {
      try {
        const [ticketRes, messagesRes] = await Promise.all([
          fetch(`http://localhost:8000/agent/tickets/${id}`, {
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

  // Setup WebSocket
  useEffect(() => {
    if (!id || !token || !user) return;

    const wsUrl = `ws://localhost:8000/ws/tickets/${id}?token=${encodeURIComponent(
      token
    )}`;
    const ws = new WebSocket(wsUrl);
    socketRef.current = ws;

    ws.onopen = () => {
      console.log("✅ WebSocket connected");
      setWsConnected(true);
    };

    ws.onmessage = (event) => {
      try {
        const newMsg = JSON.parse(event.data);
        setMessages((prev) => {
          const exists = prev.some((msg) => msg.id === newMsg.id);
          return exists ? prev : [...prev, newMsg];
        });
      } catch (err) {
        console.error("Error parsing WS message:", err);
      }
    };

    ws.onerror = (err) => {
      console.error("WebSocket error:", err);
    };

    ws.onclose = () => {
      console.warn("WebSocket closed");
      setWsConnected(false);
    };

    return () => {
      if (
        ws.readyState === WebSocket.OPEN ||
        ws.readyState === WebSocket.CONNECTING
      ) {
        ws.close();
      }
    };
  }, [id, token, user]);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  const handleSendMessage = async () => {
    if (!newMessage.trim()) return;

    const payload = {
      ticket_id: id,
      sender_id: user?.id,
      content: newMessage,
      is_from_customer: false,
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

      // 🟢 Send notification to ticket owner (customer)
      if (ticket?.user_id) {
        await fetch("http://localhost:8000/notifications", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
            Authorization: `Bearer ${token}`,
          },
          body: JSON.stringify({
            user_id: ticket.user_id,
            message: `You have a new reply from an agent on ticket #${ticket.id}`,
            link: `/dashboard/tickets/${ticket.id}`,
          }),
        });
      }

      setNewMessage(""); // Clear input
    } catch (err) {
      console.error("Message send error:", err);
    }
  };

  if (loading) return <p>Loading...</p>;
  if (error) return <p className="text-red-500">{error}</p>;
  if (!ticket) return <p>No ticket found.</p>;

  return (
    <div className="p-6 max-w-2xl mx-auto bg-white rounded-lg shadow text-black">
      <h1 className="text-2xl font-bold mb-4">🎫 Ticket #{ticket.id}</h1>

      <p className="mb-2"><strong>Subject:</strong> {ticket.subject}</p>
      <p className="mb-2"><strong>Description:</strong> {ticket.description}</p>
      <p className="mb-2"><strong>Status:</strong> {ticket.status}</p>
      <p className="mb-6">
        <strong>Created At:</strong>{" "}
        {new Date(ticket.created_at).toLocaleString()}
      </p>

      <div className="mt-6">
        <h2 className="text-xl font-semibold mb-2">Messages</h2>
        <div className="max-h-64 overflow-y-auto space-y-3 mb-3 border p-2 rounded">
          {messages.map((msg) => {
            const isFromAgent = !msg.is_from_customer;
            const isMe = isFromAgent;
            const displayName = isFromAgent ? "You (Agent)" : "Customer";
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
