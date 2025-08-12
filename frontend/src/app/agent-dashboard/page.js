'use client';

import { useEffect, useState } from 'react';
import Link from 'next/link';
import { useAuth } from '@/context/auth-context';
import useRoleGuard from '@/hooks/useRoleGuard';
import {
  Ticket,
  MessageCircle,
  ClipboardList,
  BookOpen,
  LogOut,
  CircleAlert,
} from 'lucide-react';
import LogoutButton from '@/components/LogoutButton';

export default function AgentDashboard() {
  useRoleGuard('agent');
  const { user, token } = useAuth();

  const [tickets, setTickets] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');

  useEffect(() => {
    const fetchTickets = async () => {
      try {
        const res = await fetch('http://localhost:8000/agent/tickets', {
          headers: {
            Authorization: `Bearer ${token}`,
          },
        });

        if (!res.ok) {
          throw new Error('Failed to fetch tickets');
        }

        const data = await res.json();
        setTickets(data);
      } catch (err) {
        setError(err.message);
      } finally {
        setLoading(false);
      }
    };

    if (token) fetchTickets();
  }, [token]);

  return (
    <div className="flex min-h-screen bg-gray-100 text-gray-900">
      {/* Sidebar */}
      <aside className="w-64 bg-white shadow-xl px-6 py-8 space-y-8">
        <div>
          <h2 className="text-2xl font-bold">Agent Panel</h2>
          <p className="text-sm text-gray-500 mt-1">Welcome, {user?.email}</p>
        </div>
        <nav className="space-y-4">
          <Link href="/agent-dashboard/tickets" className="flex items-center space-x-2 hover:text-blue-600">
            <Ticket className="w-5 h-5" />
            <span>Assigned Tickets</span>
          </Link>
          <Link href="/agent-dashboard/messages" className="flex items-center space-x-2 hover:text-blue-600">
            <MessageCircle className="w-5 h-5" />
            <span>User Messages</span>
          </Link>
          <Link href="/agent-dashboard/tasks" className="flex items-center space-x-2 hover:text-blue-600">
            <ClipboardList className="w-5 h-5" />
            <span>My Tasks</span>
          </Link>
          <Link href="/knowledge-base" className="flex items-center space-x-2 hover:text-blue-600">
            <BookOpen className="w-5 h-5" />
            <span>Knowledge Base</span>
          </Link>
        </nav>
        <LogoutButton className="flex items-center text-red-600 hover:underline">
          <LogOut className="w-5 h-5 mr-2" />
          Logout
        </LogoutButton>
      </aside>

      {/* Main Content */}
      <main className="flex-1 p-10">
        <h1 className="text-3xl font-bold mb-6">Hello Agent 👨‍💼</h1>

        <h2 className="text-2xl font-semibold mb-4">Assigned Tickets</h2>

        {loading ? (
          <p className="text-gray-600">Loading tickets...</p>
        ) : error ? (
          <p className="text-red-500">{error}</p>
        ) : tickets.length === 0 ? (
          <div className="flex items-center text-gray-500">
            <CircleAlert className="w-5 h-5 mr-2" />
            No tickets assigned yet.
          </div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {tickets.map((ticket) => (
              <div
                key={ticket.id}
                className="rounded-xl border border-gray-200 bg-white shadow p-5 hover:shadow-md transition"
              >
                <h3 className="text-lg font-semibold mb-1">{ticket.subject}</h3>
                <p className="text-sm text-gray-600 mb-2">{ticket.description}</p>
                <div className="flex justify-between text-sm text-gray-500">
                  <span>Status: {ticket.status}</span>
                  <span>Priority: {ticket.priority}</span>
                </div>
                <Link
                  href={`/agent-dashboard/tickets/${ticket.id}`}
                  className="text-blue-600 mt-3 inline-block text-sm font-medium hover:underline"
                >
                  View Details →
                </Link>
              </div>
            ))}
          </div>
        )}
      </main>
    </div>
  );
}
