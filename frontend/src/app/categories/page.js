"use client";

import { useEffect, useState } from "react";
import { useAuth } from "@/context/auth-context";
import { Folder } from "lucide-react"; // Optional: icon library

export default function AllCategoriesPage() {
  const { token } = useAuth();
  const [categories, setCategories] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");

  useEffect(() => {
    const fetchCategories = async () => {
      try {
        const res = await fetch("http://localhost:8000/kb/categories", {
          headers: {
            Authorization: `Bearer ${token}`,
          },
        });

        if (!res.ok) throw new Error("Failed to fetch categories");

        const data = await res.json();
        setCategories(data);
      } catch (err) {
        setError(err.message);
      } finally {
        setLoading(false);
      }
    };

    if (token) fetchCategories();
  }, [token]);

  return (
    <div className="max-w-3xl mx-auto mt-16 px-6 py-8 bg-white rounded-2xl shadow-md">
      <h1 className="text-3xl font-bold text-center text-gray-800 mb-6">
        📂 All Categories
      </h1>

      {loading ? (
        <div className="text-center text-gray-500">Loading categories...</div>
      ) : error ? (
        <div className="text-center text-red-600 bg-red-100 p-3 rounded border border-red-300">
          {error}
        </div>
      ) : categories.length === 0 ? (
        <div className="text-center text-gray-600">No categories found.</div>
      ) : (
        <ul className="grid sm:grid-cols-2 gap-4">
          {categories.map((cat) => (
            <li
              key={cat.id}
              className="flex items-center gap-3 p-4 border border-gray-200 rounded-xl bg-gray-50 hover:bg-blue-50 transition"
            >
              <Folder className="text-blue-600" />
              <span className="text-lg font-medium text-gray-800">{cat.name}</span>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
