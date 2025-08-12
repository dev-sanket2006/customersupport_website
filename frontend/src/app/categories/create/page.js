'use client';

import { useState } from 'react';
import { useAuth } from '@/context/auth-context';
import { useRouter } from 'next/navigation';
import useRoleGuard from '@/hooks/useRoleGuard';

export default function CreateCategoryPage() {
  useRoleGuard('admin');

  const { token } = useAuth();
  const router = useRouter();

  const [name, setName] = useState('');
  const [error, setError] = useState('');
  const [success, setSuccess] = useState(false);

  const handleSubmit = async (e) => {
    e.preventDefault();

    try {
      const res = await fetch('http://localhost:8000/kb/categories', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${token}`,
        },
        body: JSON.stringify({ name }),
      });

      if (!res.ok) {
        const data = await res.json();
        throw new Error(data.message || 'Failed to create category');
      }

      setSuccess(true);
      setError('');
      setName('');
      router.push('/categories');
    } catch (err) {
      setError(err.message);
      setSuccess(false);
    }
  };

  return (
    <div className="max-w-lg mx-auto mt-16 px-6 py-8 bg-white rounded-2xl shadow-lg">
      <h2 className="text-2xl font-semibold text-gray-800 mb-6 text-center">
        Create a New Category
      </h2>
      <form onSubmit={handleSubmit} className="space-y-5">
        <div>
          <label htmlFor="category-name" className="block text-sm font-medium text-gray-700 mb-1">
            Category Name
          </label>
          <input
            id="category-name"
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
            required
            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            placeholder="Enter category name"
          />
        </div>

        {error && (
          <div className="text-sm text-red-600 bg-red-100 border border-red-300 rounded p-2">
            {error}
          </div>
        )}

        {success && (
          <div className="text-sm text-green-700 bg-green-100 border border-green-300 rounded p-2">
            Category created successfully!
          </div>
        )}

        <button
          type="submit"
          className="w-full bg-blue-600 text-white py-2 rounded-lg hover:bg-blue-700 transition-all duration-200 font-medium"
        >
          Create Category
        </button>
      </form>
    </div>
  );
}
