'use client';

import { useEffect, useState } from 'react';
import { Bell } from 'lucide-react';
import { useAuth } from '@/context/auth-context';

export default function NotificationList() {
  const { token } = useAuth();
  const [notifications, setNotifications] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');

  useEffect(() => {
    if (!token) return;

    const fetchNotifications = async () => {
      try {
        const res = await fetch(`${process.env.NEXT_PUBLIC_API_BASE_URL}/notifications`, {
          headers: {
            Authorization: `Bearer ${token}`,
          },
        });

        if (!res.ok) {
          throw new Error(`Error ${res.status}: ${res.statusText}`);
        }

        const data = await res.json();
        setNotifications(data);
      } catch (err) {
        console.error('🔴 Error fetching notifications:', err);
        setError('Failed to load notifications');
      } finally {
        setLoading(false);
      }
    };

    fetchNotifications();
  }, [token]);

  const timeAgo = (timestamp) => {
    const diff = Math.floor((Date.now() - new Date(timestamp)) / 1000);
    if (diff < 60) return `${diff}s ago`;
    if (diff < 3600) return `${Math.floor(diff / 60)}m ago`;
    if (diff < 86400) return `${Math.floor(diff / 3600)}h ago`;
    return `${Math.floor(diff / 86400)}d ago`;
  };

  return (
    <section className="mb-10">
      <h2 className="text-2xl font-bold text-indigo-700 mb-4 flex items-center gap-2">
        <Bell className="w-5 h-5" /> Notifications
      </h2>

      {loading ? (
        <p className="text-gray-600 flex items-center gap-2">
          <span className="animate-spin">🔄</span> Fetching notifications...
        </p>
      ) : error ? (
        <p className="text-red-500 text-base">🚨 {error}</p>
      ) : notifications.length === 0 ? (
        <p className="text-gray-700">You have no notifications. 🎉</p>
      ) : (
        <ul className="space-y-3">
          {notifications.map((note) => (
            <li
              key={note.id}
              className={`rounded-lg p-4 border-l-4 shadow-sm ${
                note.is_read
                  ? 'bg-gray-100 border-gray-400'
                  : 'bg-yellow-50 border-yellow-500'
              }`}
            >
              <div className="flex justify-between items-start gap-3">
                <div className="flex-1">
                  <p className="text-gray-800 font-medium">{note.message}</p>
                  {note.link && (
                    <a
                      href={note.link}
                      className="text-sm text-blue-600 hover:underline"
                    >
                      🔗 View
                    </a>
                  )}
                </div>
                <span className="text-xs text-gray-500 whitespace-nowrap">
                  {timeAgo(note.created_at)}
                </span>
              </div>
            </li>
          ))}
        </ul>
      )}
    </section>
  );
}
