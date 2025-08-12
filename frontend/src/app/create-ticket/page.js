'use client';

import { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation'; // ✅ Import router
import { useAuth } from '@/context/auth-context';

export default function CreateTicketPage() {
  const router = useRouter(); // ✅ Init router
  const { user } = useAuth();
  const [subject, setSubject] = useState('');
  const [description, setDescription] = useState('');
  const [priority, setPriority] = useState('medium'); // ✅ lowercase
  const [email, setEmail] = useState('');
  const [message, setMessage] = useState('');
  const [status, setStatus] = useState('');
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (user?.email) {
      setEmail(user.email);
    }
  }, [user]);

  const handleSubmit = async (e) => {
    e.preventDefault();
    setLoading(true);
    setMessage('');
    setStatus('');

    const token = localStorage.getItem('token');
    if (!token) {
      setMessage('❌ You must be logged in.');
      setStatus('error');
      setLoading(false);
      return;
    }

    try {
      const res = await fetch('http://localhost:8000/tickets', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${token}`,
        },
        body: JSON.stringify({
          subject,
          description,
          priority,
          customer_email: email,
        }),
      });

      if (res.ok) {
        setMessage('✅ Ticket created successfully!');
        setStatus('success');
        setSubject('');
        setDescription('');
        setPriority('medium');

        setTimeout(() => {
          router.push('/dashboard'); // ✅ Redirect
        }, 1000);
      } else {
        const errorText = await res.text();
        console.error('Error:', errorText);
        setMessage('❌ Failed to create ticket.');
        setStatus('error');
      }
    } catch (err) {
      console.error('Network error:', err);
      setMessage('❌ Network error occurred.');
      setStatus('error');
    } finally {
      setLoading(false);
    }
  };

  return (
    <main className="p-4 sm:p-8 max-w-xl mx-auto">
      <h1 className="text-2xl font-bold mb-6 text-center text-gray-800 dark:text-white">
        Create New Ticket
      </h1>

      <form
        onSubmit={handleSubmit}
        className="space-y-4 bg-white dark:bg-gray-800 p-6 rounded-xl shadow-md border border-gray-200 dark:border-gray-700"
      >
        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Subject</label>
          <input
            type="text"
            value={subject}
            onChange={(e) => setSubject(e.target.value)}
            required
            className="w-full border px-4 py-2 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
            placeholder="e.g. Account Issue"
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Description</label>
          <textarea
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            required
            rows={4}
            className="w-full border px-4 py-2 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
            placeholder="Describe the issue..."
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Priority</label>
          <select
            value={priority}
            onChange={(e) => setPriority(e.target.value)}
            className="w-full border px-4 py-2 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
          >
            <option value="low">Low</option>
            <option value="medium">Medium</option>
            <option value="high">High</option>
          </select>
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Your Email</label>
          <input
            type="email"
            value={email}
            readOnly
            className="w-full border px-4 py-2 rounded-md bg-gray-100 dark:bg-gray-700 text-gray-900 dark:text-white cursor-not-allowed"
            placeholder="user@example.com"
          />
        </div>

        <button
          type="submit"
          disabled={loading}
          className="w-full bg-blue-600 hover:bg-blue-700 text-white py-2 rounded-lg font-medium transition disabled:opacity-50"
        >
          {loading ? 'Submitting...' : 'Create Ticket'}
        </button>
      </form>

      {message && (
        <p
          className={`mt-4 text-center font-medium ${
            status === 'success' ? 'text-green-600 dark:text-green-400' : 'text-red-600 dark:text-red-400'
          }`}
        >
          {message}
        </p>
      )}
    </main>
  );
}
