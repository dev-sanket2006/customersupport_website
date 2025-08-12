"use client";

import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import { useAuth } from "@/context/auth-context";
import useRoleGuard from "@/hooks/useRoleGuard";

export default function AgentTickets() {
  // ✅ FIXED: Pass array instead of string
  useRoleGuard(["agent"]);
  
  const { token } = useAuth();
  const [tickets, setTickets] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const router = useRouter();

  useEffect(() => {
    const fetchTickets = async () => {
      try {
        // ✅ GOOD: Already using correct endpoint
        const res = await fetch(`http://localhost:8000/agent/tickets`, {
          headers: {
            Authorization: `Bearer ${token}`,
          },
        });

        if (!res.ok) {
          const errText = await res.text();
          throw new Error(errText || "Failed to load tickets");
        }

        const data = await res.json();
        setTickets(data);
      } catch (err) {
        console.error(err);
        setError("Error fetching tickets. Please try again.");
      } finally {
        setLoading(false);
      }
    };

    if (token) fetchTickets();
  }, [token]);

  if (loading) return <p className="p-6">Loading tickets...</p>;
  if (error) return <p className="p-6 text-red-500">{error}</p>;

  return (
    <div className="p-8 max-w-4xl mx-auto">
      <h1 className="text-2xl font-bold mb-6">Assigned Tickets</h1>
      {tickets.length === 0 ? (
        <p>No tickets assigned to you.</p>
      ) : (
        <div className="space-y-4">
          {tickets.map((ticket) => (
            <div
              key={ticket.id}
              className="p-4 border rounded-md bg-white shadow-sm hover:bg-gray-50 cursor-pointer"
              // ✅ FIXED: Corrected route path
              onClick={() => router.push(`/agent-dashboard/tickets/${ticket.id}`)}
            >
              <h2 className="font-semibold">{ticket.subject}</h2>
              <p className="text-sm text-gray-600 capitalize">
                {ticket.status} - {ticket.priority}
              </p>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}