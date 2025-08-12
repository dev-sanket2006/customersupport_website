"use client";

import { useEffect, useState } from "react";
import { useAuth } from "@/context/auth-context";

export default function TicketListWithNotes() {
  const { user } = useAuth();
  const [tickets, setTickets] = useState([]);
  const [messages, setMessages] = useState({});
  const [newMessages, setNewMessages] = useState({});
  const [showMessages, setShowMessages] = useState({});
  const [sockets, setSockets] = useState({});

  const token =
    typeof window !== "undefined" ? localStorage.getItem("token") : null;

  useEffect(() => {
    if (!token || !user?.email) return;

    fetch(`http://localhost:8000/tickets`, {
      headers: { Authorization: `Bearer ${token}` },
    })
      .then((res) => res.json())
      .then((data) => {
        const userTickets = data.filter((t) => t.customer_email === user.email);
        setTickets(userTickets);
      })
      .catch((err) => console.error("Error fetching tickets:", err));
  }, [token, user]);

  const connectWebSocket = (ticketId) => {
    if (sockets[ticketId]) return;
    const ws = new WebSocket(`ws://localhost:8000/ws/tickets/${id}`, [token]); // ✅ correct path

    ws.onmessage = (event) => {
      const msg = JSON.parse(event.data);
      setMessages((prev) => ({
        ...prev,
        [ticketId]: [msg, ...(prev[ticketId] || [])],
      }));
    };

    ws.onerror = (e) => {
      console.error("WebSocket error", e);
    };

    setSockets((prev) => ({ ...prev, [ticketId]: ws }));
  };

  const toggleMessages = (ticketId) => {
    setShowMessages((prev) => ({ ...prev, [ticketId]: !prev[ticketId] }));
    if (!sockets[ticketId]) connectWebSocket(ticketId);
  };

  const handleSendMessage = (ticketId) => {
    const content = newMessages[ticketId];
    if (!content?.trim()) return;

    const msg = {
      sender_id: user.id,
      sender_type: "customer",
      content,
      timestamp: new Date().toISOString(),
    };

    if (sockets[ticketId]?.readyState === WebSocket.OPEN) {
      sockets[ticketId].send(JSON.stringify(msg));
      setNewMessages((prev) => ({ ...prev, [ticketId]: "" }));
    } else {
      console.error("WebSocket not connected");
    }
  };

  return (
    <main className="p-4 max-w-3xl mx-auto">
      <h1 className="text-2xl font-bold mb-6">Your Tickets</h1>

      {tickets.length === 0 ? (
        <p>No tickets found.</p>
      ) : (
        tickets.map((ticket) => (
          <div
            key={ticket.id}
            className="border rounded-xl p-4 mb-6 bg-white dark:bg-gray-800 shadow"
          >
            <h2 className="text-lg font-semibold text-gray-900 dark:text-white">
              {ticket.subject}
            </h2>
            <p className="text-sm text-gray-600 dark:text-gray-300">
              {ticket.description}
            </p>
            <p className="text-sm text-gray-500 mt-1">
              Priority: {ticket.priority} | Status: {ticket.status}
            </p>

            <button
              onClick={() => toggleMessages(ticket.id)}
              className="mt-4 text-blue-600 hover:underline"
            >
              {showMessages[ticket.id]
                ? "Hide Conversation"
                : "View Conversation"}
            </button>

            {showMessages[ticket.id] && (
              <div className="mt-4">
                <div className="space-y-3 max-h-48 overflow-y-auto mb-4">
                  {(messages[ticket.id] || []).map((msg, idx) => (
                    <div
                      key={idx}
                      className={`p-2 rounded ${
                        msg.sender_type === "customer"
                          ? "bg-blue-100 dark:bg-blue-800 text-right"
                          : "bg-gray-100 dark:bg-gray-700 text-left"
                      }`}
                    >
                      <p className="text-sm text-gray-800 dark:text-white">
                        {msg.content}
                      </p>
                      <p className="text-xs text-gray-500 mt-1">
                        {msg.sender_type === "customer"
                          ? "You"
                          : "Support Agent"}
                        , {new Date(msg.timestamp).toLocaleString()}
                      </p>
                    </div>
                  ))}
                </div>

                <textarea
                  rows={2}
                  value={newMessages[ticket.id] || ""}
                  onChange={(e) =>
                    setNewMessages((prev) => ({
                      ...prev,
                      [ticket.id]: e.target.value,
                    }))
                  }
                  className="w-full border rounded p-2 text-sm dark:bg-gray-700 dark:text-white"
                  placeholder="Type a message..."
                />
                <button
                  onClick={() => handleSendMessage(ticket.id)}
                  className="mt-2 px-4 py-1 bg-blue-600 text-white rounded hover:bg-blue-700"
                >
                  Send
                </button>
              </div>
            )}
          </div>
        ))
      )}
    </main>
  );
}
