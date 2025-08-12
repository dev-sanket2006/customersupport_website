'use client';

import { useState } from 'react';

export default function CreateNotePage() {
  const [ticketId, setTicketId] = useState('');
  const [content, setContent] = useState('');
  const [message, setMessage] = useState('');
  const [status, setStatus] = useState('');
  const [loading, setLoading] = useState(false);

  async function handleSubmit(e) {
    e.preventDefault();
    setLoading(true);
    setMessage('');
    setStatus('');

    const token = localStorage.getItem('token');
    if (!token) {
      setMessage('❌ Not logged in.');
      setStatus('error');
      setLoading(false);
      return;
    }

    try {
      const res = await fetch('http://localhost:8000/notes', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${token}`,
        },
        body: JSON.stringify({
          ticket_id: ticketId,
          content,
        }),
      });

      if (res.ok) {
        setMessage('✅ Note created successfully!');
        setStatus('success');
        setTicketId('');
        setContent('');
      } else {
        const errorText = await res.text();
        console.error('Error:', errorText);
        setMessage('❌ Failed to create note.');
        setStatus('error');
      }
    } catch (err) {
      console.error('Network error:', err);
      setMessage('❌ Network error occurred.');
      setStatus('error');
    } finally {
      setLoading(false);
    }
  }

  return (
    <main className="p-4 sm:p-8 max-w-xl mx-auto">
      <h1 className="text-2xl font-bold mb-6 text-center text-gray-800 dark:text-white">Create a Note</h1>

      <form
        onSubmit={handleSubmit}
        className="space-y-4 bg-white dark:bg-gray-800 p-6 rounded-xl shadow-md border border-gray-200 dark:border-gray-700"
      >
        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Ticket ID (UUID)</label>
          <input
            type="text"
            value={ticketId}
            onChange={(e) => setTicketId(e.target.value)}
            required
            className="w-full border border-gray-300 dark:border-gray-600 px-4 py-2 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:outline-none"
            placeholder="e.g. 123e4567-e89b-12d3-a456-426614174000"
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Note Content</label>
          <textarea
            value={content}
            onChange={(e) => setContent(e.target.value)}
            required
            rows={5}
            className="w-full border border-gray-300 dark:border-gray-600 px-4 py-2 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:outline-none"
            placeholder="Write your note here..."
          />
        </div>

        <button
          type="submit"
          disabled={loading}
          className="w-full bg-blue-600 hover:bg-blue-700 text-white py-2 rounded-lg font-medium transition disabled:opacity-50"
        >
          {loading ? 'Submitting...' : 'Submit'}
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
