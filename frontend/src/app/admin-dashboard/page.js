"use client";

import { useEffect, useState, useRef } from "react";
import Link from "next/link";
import useRoleGuard from "@/hooks/useRoleGuard";
import { useAuth } from "@/context/auth-context";
import LogoutButton from "@/components/LogoutButton";
import ThemeToggle from "@/components/ThemeToggle";

export default function AdminDashboard() {
  useRoleGuard("admin");
  const { user } = useAuth();

  const [tickets, setTickets] = useState([]);
  const [agents, setAgents] = useState([]);
  const [messagesByTicket, setMessagesByTicket] = useState({});
  const [visibleMessages, setVisibleMessages] = useState({});
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(true);
  const [assigning, setAssigning] = useState(null);

  const wsConnections = useRef({});
  const token =
    typeof window !== "undefined" ? localStorage.getItem("token") : null;

  useEffect(() => {
    if (!token) return;

    const fetchData = async () => {
      setLoading(true);
      setError("");
      try {
        const [ticketRes, agentRes] = await Promise.all([
          fetch("http://localhost:8000/admin/tickets", {
            headers: { Authorization: `Bearer ${token}` },
          }),
          fetch("http://localhost:8000/agents", {
            headers: { Authorization: `Bearer ${token}` },
          }),
        ]);

        if (!ticketRes.ok) throw new Error("Failed to fetch tickets");
        if (!agentRes.ok) throw new Error("Failed to fetch agents");

        const ticketsData = await ticketRes.json();
        const agentsData = await agentRes.json();

        setTickets(ticketsData);
        setAgents(agentsData);
      } catch (err) {
        setError(err.message);
      } finally {
        setLoading(false);
      }
    };

    fetchData();

    return () => {
      const currentConnections = wsConnections.current;
      Object.values(currentConnections).forEach((ws) => {
        if (ws && ws.readyState === WebSocket.OPEN) {
          ws.close();
        }
      });
    };
  }, [token]);

  const assignAgent = async (ticketId, agentId) => {
    if (!agentId) return;
    setAssigning(ticketId);

    try {
      const res = await fetch(
        `http://localhost:8000/tickets/assign/${ticketId}/${agentId}`,
        {
          method: "PUT",
          headers: { Authorization: `Bearer ${token}` },
        }
      );

      if (!res.ok) throw new Error("Failed to assign agent");

      const updated = await fetch("http://localhost:8000/admin/tickets", {
        headers: { Authorization: `Bearer ${token}` },
      });
      const data = await updated.json();
      setTickets(data);
      alert("✅ Agent assigned successfully");
    } catch (err) {
      alert("❌ " + err.message);
    } finally {
      setAssigning(null);
    }
  };

  const toggleMessages = async (ticketId) => {
    setVisibleMessages((prev) => ({ ...prev, [ticketId]: !prev[ticketId] }));

    if (!visibleMessages[ticketId]) {
      if (!messagesByTicket[ticketId]) {
        try {
          const res = await fetch(
            `http://localhost:8000/messages/${ticketId}`,
            {
              headers: { Authorization: `Bearer ${token}` },
            }
          );

          if (!res.ok) throw new Error("Failed to fetch messages");

          const msgs = await res.json();
          setMessagesByTicket((prev) => ({ ...prev, [ticketId]: msgs }));
        } catch (err) {
          alert(`❌ Failed to load messages: ${err.message}`);
        }
      }

      if (!wsConnections.current[ticketId]) {
        const ws = new WebSocket(
          `ws://localhost:8000/ws/tickets/${ticketId}?token=${encodeURIComponent(
            token
          )}`
        );

        ws.onmessage = (event) => {
          try {
            const message = JSON.parse(event.data);
            setMessagesByTicket((prev) => ({
              ...prev,
              [ticketId]: [...(prev[ticketId] || []), message],
            }));
          } catch (e) {
            console.error("Failed to parse incoming WebSocket message", e);
          }
        };

        ws.onerror = (err) => {
          console.error(`WebSocket error for ticket ${ticketId}`, err);
        };

        ws.onclose = () => {
          console.log(`WebSocket closed for ticket ${ticketId}`);
        };

        wsConnections.current[ticketId] = ws;
      }
    }
  };

  return (
    <div className="p-6 text-gray-900 dark:text-gray-100 bg-white dark:bg-gray-900 min-h-screen transition-colors">
      <div className="flex justify-between items-start flex-wrap gap-4">
        <div>
          <h1 className="text-3xl font-bold mb-2">Admin Dashboard</h1>
          <p className="text-gray-600 dark:text-gray-300">
            Welcome, <strong>{user?.email}</strong>
          </p>
          <p className="text-sm text-gray-500 dark:text-gray-400">
            Role: {user?.role}
          </p>
        </div>

        <div className="flex gap-2 flex-wrap">
          <Link
            href="/knowledge-base"
            className="bg-green-600 text-white px-4 py-2 rounded hover:bg-green-700 text-sm transition-colors"
          >
            📚 Knowledge Base
          </Link>

          <Link
            href="/categories/create"
            className="bg-purple-600 text-white px-4 py-2 rounded hover:bg-purple-700 text-sm transition-colors"
          >
            ➕ Create Category
          </Link>

          <LogoutButton />
          <ThemeToggle />
        </div>
      </div>

      <hr className="my-6 border-gray-300 dark:border-gray-700" />

      <h2 className="text-2xl font-semibold mb-4">All Tickets</h2>

      {loading ? (
        <p className="animate-pulse text-gray-500">Loading tickets...</p>
      ) : error ? (
        <p className="text-red-500 dark:text-red-400">❌ {error}</p>
      ) : tickets.length === 0 ? (
        <p className="text-gray-500 dark:text-gray-400">
          No tickets available.
        </p>
      ) : (
        <div className="space-y-5">
          {tickets.map((ticket) => (
            <div
              key={ticket.id}
              className="rounded-xl border border-gray-300 dark:border-gray-700 p-5 shadow-sm bg-gray-50 dark:bg-gray-800 transition-colors"
            >
              <p className="font-semibold text-lg text-indigo-600 dark:text-indigo-400">
                🎫 {ticket.subject}
              </p>
              <p className="text-sm mt-1 text-gray-700 dark:text-gray-300">
                Status: <span className="font-medium">{ticket.status}</span> |
                Priority: <span className="font-medium">{ticket.priority}</span>
              </p>
              <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">
                Assigned to:{" "}
                <span className="italic">
                  {ticket.assigned_to || "Unassigned"}
                </span>
              </p>

              <div className="mt-3 flex flex-col sm:flex-row sm:items-center gap-3">
                <div>
                  <label className="text-sm mr-2">Assign Agent:</label>
                  <select
                    className="border dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-100 px-2 py-1 rounded"
                    defaultValue=""
                    onChange={(e) => assignAgent(ticket.id, e.target.value)}
                    disabled={assigning === ticket.id}
                  >
                    <option value="" disabled>
                      {assigning === ticket.id
                        ? "Assigning..."
                        : "Select agent"}
                    </option>
                    {agents.map((agent) => (
                      <option key={agent.id} value={agent.id}>
                        {agent.name || agent.email}
                      </option>
                    ))}
                  </select>
                </div>

                <button
                  onClick={() => toggleMessages(ticket.id)}
                  className="bg-indigo-100 dark:bg-indigo-700 text-indigo-800 dark:text-white border border-indigo-300 dark:border-indigo-600 px-3 py-1 rounded hover:bg-indigo-200 dark:hover:bg-indigo-600 text-sm"
                >
                  {visibleMessages[ticket.id]
                    ? "Hide Messages"
                    : "View Messages"}
                </button>
              </div>

              {visibleMessages[ticket.id] && (
                <div className="mt-4 bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-700 rounded p-4 text-sm transition-colors max-h-80 overflow-y-auto">
                  {messagesByTicket[ticket.id] ? (
                    messagesByTicket[ticket.id].length === 0 ? (
                      <p className="text-gray-500 dark:text-gray-400">
                        No messages yet.
                      </p>
                    ) : (
                      <ul className="space-y-3">
                        {messagesByTicket[ticket.id].map((msg) => (
                          <li
                            key={msg.id}
                            className="border-b border-gray-200 dark:border-gray-700 pb-2"
                          >
                            <p className="flex items-center gap-2">
                              <span
                                className={`px-2 py-0.5 text-xs rounded-full font-semibold ${
                                  msg.is_from_customer
                                    ? "bg-blue-100 text-blue-800 dark:bg-blue-800 dark:text-white"
                                    : "bg-green-100 text-green-800 dark:bg-green-800 dark:text-white"
                                }`}
                              >
                                {msg.is_from_customer ? "User" : "Agent"}
                              </span>
                              <span className="text-sm font-medium text-gray-800 dark:text-gray-200">
                                {msg.sender_name || msg.sender_id}
                              </span>
                            </p>
                            <p className="mt-1 text-gray-700 dark:text-gray-300">
                              {msg.content}
                            </p>
                            <p className="text-gray-400 text-xs mt-0.5">
                              {new Date(msg.created_at).toLocaleString()}
                            </p>
                          </li>
                        ))}
                      </ul>
                    )
                  ) : (
                    <p className="text-gray-400">Loading messages...</p>
                  )}
                </div>
              )}
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
