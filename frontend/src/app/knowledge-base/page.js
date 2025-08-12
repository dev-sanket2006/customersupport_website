"use client";

import { useEffect, useState } from "react";
import Link from "next/link";
import { useAuth } from "@/context/auth-context";
import { useSearchParams, useRouter } from "next/navigation";
import { FileText, Plus } from "lucide-react";

export default function KBListPage() {
  const { token, user } = useAuth();
  const searchParams = useSearchParams();
  const router = useRouter();

  const [articles, setArticles] = useState([]);
  const [categories, setCategories] = useState([]);
  const [tags, setTags] = useState([]);
  const [search, setSearch] = useState("");
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");

  const selectedCategory = searchParams.get("cat");
  const selectedTag = searchParams.get("tag");

  useEffect(() => {
    const fetchFilters = async () => {
      if (!token) return;

      try {
        const [catRes, tagRes] = await Promise.all([
          fetch("http://localhost:8000/kb/categories", {
            headers: { Authorization: `Bearer ${token}` },
          }),
          fetch("http://localhost:8000/kb/tags", {
            headers: { Authorization: `Bearer ${token}` },
          }),
        ]);

        const catData = await catRes.json();
        const tagData = await tagRes.json();

        setCategories(catData);
        setTags(tagData);
      } catch (err) {
        console.error("Failed to fetch filters:", err);
      }
    };
    fetchFilters();
  }, [token]);

  useEffect(() => {
    const fetchArticles = async () => {
      if (!token) return;

      try {
        let url = "http://localhost:8000/kb/articles";
        if (selectedCategory)
          url = `http://localhost:8000/kb/categories/${selectedCategory}/articles`;
        else if (selectedTag)
          url = `http://localhost:8000/kb/articles/tag/${selectedTag}`;

        const res = await fetch(url, {
          headers: {
            Authorization: `Bearer ${token}`,
          },
        });

        if (!res.ok) throw new Error("Failed to fetch articles");

        const data = await res.json();
        setArticles(data);
      } catch (err) {
        console.error(err);
        setError("Failed to load articles.");
      } finally {
        setLoading(false);
      }
    };

    fetchArticles();
  }, [token, selectedCategory, selectedTag]);

  const handleSearch = (e) => {
    setSearch(e.target.value);
  };

  const filteredArticles = articles.filter((article) =>
    article.title.toLowerCase().includes(search.toLowerCase())
  );

  const updateQuery = (key, value) => {
    const params = new URLSearchParams(searchParams);
    if (value) params.set(key, value);
    else params.delete(key);
    router.push(`/knowledge-base?${params.toString()}`);
  };

  return (
    <div className="max-w-7xl mx-auto px-6 py-10">
      <div className="flex flex-col sm:flex-row justify-between items-center gap-4 mb-8">
        <h1 className="text-4xl font-bold bg-gradient-to-r from-amber-500 via-pink-500 to-red-500 bg-clip-text text-transparent">
          📚 Knowledge Base
        </h1>

        {(user?.role === "admin" || user?.role === "agent") && (
          <Link
            href="/knowledge-base/create"
            className="inline-flex items-center gap-2 bg-amber-500 text-white px-5 py-2.5 rounded-lg hover:bg-amber-600 shadow-md transition"
          >
            <Plus className="w-4 h-4" />
            Create Article
          </Link>
        )}
      </div>

      {/* Filters */}
      <div className="flex flex-wrap gap-4 mb-6">
        <select
          value={selectedCategory || ""}
          onChange={(e) => updateQuery("cat", e.target.value)}
          className="px-4 py-2 rounded-lg border border-gray-300 bg-white shadow-sm text-gray-800 focus:ring-2 focus:ring-amber-500"
        >
          <option value="">All Categories</option>
          {categories.map((cat) => (
            <option key={cat.id} value={cat.id}>
              {cat.name}
            </option>
          ))}
        </select>

        <select
          value={selectedTag || ""}
          onChange={(e) => updateQuery("tag", e.target.value)}
          className="px-4 py-2 rounded-lg border border-gray-300 bg-white shadow-sm text-gray-800 focus:ring-2 focus:ring-amber-500"
        >
          <option value="">All Tags</option>
          {tags.map((tag) => {
            const key = typeof tag === "string" ? tag : tag.id || tag.name;
            const value = typeof tag === "string" ? tag : tag.name;
            return (
              <option key={key} value={value}>
                {value}
              </option>
            );
          })}
        </select>

        <input
          type="text"
          value={search}
          onChange={handleSearch}
          placeholder="Search articles..."
          className="flex-1 min-w-[200px] px-4 py-2 border border-gray-300 rounded-lg shadow-sm focus:ring-2 focus:ring-amber-500 text-gray-800"
        />
      </div>

      {/* Loading / Error / Empty states */}
      {loading && (
        <p className="text-center text-amber-600">Loading articles...</p>
      )}
      {error && (
        <p className="text-center text-red-600 bg-red-100 p-3 rounded border border-red-300">
          {error}
        </p>
      )}
      {!loading && filteredArticles.length === 0 && (
        <p className="text-center text-gray-500">No articles found.</p>
      )}

      {/* Article cards */}
      <ul className="grid gap-6 sm:grid-cols-2 md:grid-cols-3">
        {filteredArticles.map((article) => {
          const isAdminOrAgent =
            user?.role === "admin" || user?.role === "agent";
          const articleUrl = isAdminOrAgent
            ? `/knowledge-base/${article.id}`
            : `/knowledge-base/view/${article.id}`;

          return (
            <li
              key={article.id}
              className="border border-gray-200 rounded-2xl p-5 bg-gradient-to-br from-white to-amber-50 shadow-lg hover:shadow-xl transition"
            >
              <div className="flex justify-between items-start mb-3">
                <div className="flex items-center gap-2">
                  <FileText className="text-amber-600 w-5 h-5" />
                  <h2 className="text-lg font-semibold text-gray-800">
                    {article.title}
                  </h2>
                </div>
                <Link
                  href={articleUrl}
                  className="text-sm text-amber-600 hover:underline"
                >
                  {isAdminOrAgent ? "View / Edit" : "View"}
                </Link>
              </div>

              <div className="mt-2 flex flex-wrap gap-2 text-sm">
                <span className="px-2 py-1 bg-purple-100 text-purple-700 rounded-full">
                  Category ID: {article.category_id}
                </span>
                <span
                  className={`px-2 py-1 rounded-full ${
                    article.is_published
                      ? "bg-green-100 text-green-700"
                      : "bg-yellow-100 text-yellow-800"
                  }`}
                >
                  {article.is_published ? "Published" : "Draft"}
                </span>

                {article.tags?.length > 0 ? (
                  article.tags.map((tag) => (
                    <button
                      key={tag}
                      onClick={() => updateQuery("tag", tag)}
                      className="px-2 py-1 bg-amber-100 text-amber-700 rounded-full hover:bg-amber-200 transition"
                    >
                      #{tag}
                    </button>
                  ))
                ) : (
                  <span className="text-gray-400 italic">
                    No tags —{" "}
                    <Link
                      href={`/knowledge-base/${article.id}`}
                      className="text-blue-500 underline"
                    >
                      Add tag
                    </Link>
                  </span>
                )}
              </div>
            </li>
          );
        })}
      </ul>
    </div>
  );
}
