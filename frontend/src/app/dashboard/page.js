"use client";

import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import useRoleGuard from "@/hooks/useRoleGuard";
import { useAuth } from "@/context/auth-context";
import LogoutButton from "@/components/LogoutButton";
import { FileText, Bell } from "lucide-react";

export default function UserDashboard() {
  useRoleGuard("user");
  const { user } = useAuth();
  const router = useRouter();

  const [tickets, setTickets] = useState([]);
  const [articles, setArticles] = useState([]);
  const [filteredArticles, setFilteredArticles] = useState([]);
  const [categories, setCategories] = useState([]);
  const [tags, setTags] = useState([]);
  const [notifications, setNotifications] = useState([]);

  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [search, setSearch] = useState("");
  const [selectedCategory, setSelectedCategory] = useState("");
  const [selectedTag, setSelectedTag] = useState("");

  useEffect(() => {
    const token = localStorage.getItem("token");
    if (!token) {
      setError("Not authenticated.");
      setLoading(false);
      return;
    }

    const fetchData = async () => {
      try {
        const [
          ticketsRes,
          articlesRes,
          categoriesRes,
          tagsRes,
          notificationsRes,
        ] = await Promise.all([
          fetch("http://localhost:8000/tickets", {
            headers: { Authorization: `Bearer ${token}` },
          }),
          fetch("http://localhost:8000/kb/articles"),
          fetch("http://localhost:8000/kb/categories"),
          fetch("http://localhost:8000/kb/tags"),
          fetch("http://localhost:8000/notifications", {
            headers: { Authorization: `Bearer ${token}` },
          }),
        ]);

        if (
          !ticketsRes.ok ||
          !articlesRes.ok ||
          !categoriesRes.ok ||
          !tagsRes.ok ||
          !notificationsRes.ok
        ) {
          throw new Error("Failed to load one or more resources");
        }

        const ticketsData = await ticketsRes.json();
        const articlesData = (await articlesRes.json()).filter(
          (a) => a.is_published
        );
        const categoriesData = await categoriesRes.json();
        const tagsData = await tagsRes.json();
        const notificationsData = await notificationsRes.json();

        setTickets(ticketsData);
        setArticles(articlesData);
        setFilteredArticles(articlesData);
        setCategories(categoriesData);
        setTags(tagsData);
        setNotifications(
          notificationsData.sort(
            (a, b) => new Date(b.created_at) - new Date(a.created_at)
          )
        );
      } catch (err) {
        setError(err.message);
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, [user]);

  useEffect(() => {
    let filtered = [...articles];

    if (search.trim() !== "") {
      filtered = filtered.filter((a) =>
        a.title.toLowerCase().includes(search.trim().toLowerCase())
      );
    }

    if (selectedCategory) {
      filtered = filtered.filter((a) => a.category_id === selectedCategory);
    }

    if (selectedTag) {
      filtered = filtered.filter((a) =>
        a.tags?.some(
          (t) => (typeof t === "string" ? t : t.name) === selectedTag
        )
      );
    }

    setFilteredArticles(filtered);
  }, [search, selectedCategory, selectedTag, articles]);

  function timeAgo(date) {
    const diff = Math.floor((new Date() - new Date(date)) / 1000);
    if (diff < 60) return `${diff} seconds ago`;
    if (diff < 3600) return `${Math.floor(diff / 60)} minutes ago`;
    if (diff < 86400) return `${Math.floor(diff / 3600)} hours ago`;
    return `${Math.floor(diff / 86400)} days ago`;
  }

  return (
    <div className="min-h-screen bg-gradient-to-br from-yellow-50 via-teal-50 to-sky-100 p-6 font-[ui-rounded]">
      <div className="max-w-7xl mx-auto bg-white rounded-3xl shadow-2xl p-10 border-2 border-yellow-400">
        <h1 className="text-4xl font-black text-orange-600 text-center mb-6">
          🎮 Welcome to Your Playful Dashboard!
        </h1>

        <p className="text-lg mb-1 text-sky-900 text-center">
          Logged in as{" "}
          <span className="font-bold text-indigo-600">{user?.email}</span>
        </p>
        <p className="text-base text-sky-700 text-center mb-6">
          🧠 Role: <span className="font-semibold">{user?.role}</span>
        </p>

        <div className="flex flex-col sm:flex-row justify-center gap-4 mb-6">
          <button
            onClick={() => router.push("/create-ticket")}
            className="bg-orange-500 hover:bg-orange-600 text-white px-5 py-2 rounded-full font-semibold shadow-md transition"
          >
            ➕ Let’s Create a Ticket!
          </button>
          <LogoutButton />
        </div>

        {/* 🔔 Notifications */}
        <h2 className="text-2xl font-bold text-indigo-700 mb-4 flex items-center gap-2">
          <Bell className="w-5 h-5" /> Notifications
        </h2>

        {loading ? (
          <p className="text-gray-600 text-base">Fetching notifications...</p>
        ) : error ? (
          <p className="text-red-500 text-base">🚨 {error}</p>
        ) : notifications.length === 0 ? (
          <p className="text-gray-700 text-base">You have no notifications.</p>
        ) : (
          <ul className="mb-10 space-y-3">
            {notifications.map((note) => (
              <li
                key={note.id}
                className={`border-l-4 ${
                  note.is_read ? "border-gray-400" : "border-green-500"
                } bg-gray-50 px-4 py-3 rounded shadow`}
              >
                <p className="text-gray-800">
                  {note.message}{" "}
                  {note.link && (
                    <a
                      href={note.link}
                      className="text-blue-600 hover:underline ml-1"
                    >
                      View
                    </a>
                  )}
                </p>
                <p className="text-sm text-gray-500">
                  {timeAgo(note.created_at)}
                </p>
              </li>
            ))}
          </ul>
        )}

        <hr className="my-8 border-teal-300" />

        <h2 className="text-2xl font-bold text-sky-800 mb-4">
          🎟️ Your Tickets
        </h2>

        {loading ? (
          <p className="text-gray-600 text-base">Loading awesomeness...</p>
        ) : error ? (
          <p className="text-red-500 text-base">💥 {error}</p>
        ) : tickets.length === 0 ? (
          <p className="text-gray-700 text-base">
            No tickets yet. Time to break something? 😜
          </p>
        ) : (
          <ul className="grid md:grid-cols-2 gap-4">
            {tickets.map((ticket) => (
              <li
                key={ticket.id}
                className="bg-white border-l-4 border-lime-500 rounded-xl p-4 shadow-sm hover:shadow-lg transition"
              >
                <p className="font-semibold text-orange-700 text-lg">
                  🎯 {ticket.subject}
                </p>
                <p className="text-base text-gray-700 mt-1">
                  📌 Status: <span className="capitalize">{ticket.status}</span>{" "}
                  | ⚡ Priority:{" "}
                  <span className="capitalize">{ticket.priority}</span>
                </p>
                <p className="text-base text-gray-600 mt-1">
                  {ticket.description}
                </p>
                <button
                  onClick={() => router.push(`/dashboard/tickets/${ticket.id}`)}
                  className="mt-2 text-blue-600 hover:underline text-base"
                >
                  📨 Peek Messages
                </button>
              </li>
            ))}
          </ul>
        )}

        <hr className="my-10 border-sky-300" />

        <h2 className="text-2xl font-bold text-orange-600 mb-4">
          📚 Explore Knowledge Base
        </h2>

        {/* 🔍 Filters */}
        <div className="flex flex-col sm:flex-row gap-4 mb-6">
          <input
            type="text"
            placeholder="🔍 Search titles..."
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            className="w-full sm:w-1/3 border-2 border-orange-400 px-4 py-2 rounded-md text-base text-black font-medium bg-yellow-100 placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-orange-500 shadow-sm"
          />

          <select
            value={selectedCategory}
            onChange={(e) => setSelectedCategory(e.target.value)}
            className="w-full sm:w-1/3 border-2 border-lime-400 px-4 py-2 rounded-md text-base text-black font-medium bg-lime-100 focus:outline-none focus:ring-2 focus:ring-lime-500 shadow-sm"
          >
            <option value="">📁 All Categories</option>
            {categories.map((cat) => (
              <option key={cat.id} value={cat.id}>
                {cat.name}
              </option>
            ))}
          </select>

          <select
            value={selectedTag}
            onChange={(e) => setSelectedTag(e.target.value)}
            className="w-full sm:w-1/3 border-2 border-sky-400 px-4 py-2 rounded-md text-base text-black font-medium bg-sky-100 focus:outline-none focus:ring-2 focus:ring-sky-500 shadow-sm"
          >
            <option value="">🏷️ All Tags</option>
            {tags.map((tag) => (
              <option
                key={typeof tag === "string" ? tag : tag.id}
                value={typeof tag === "string" ? tag : tag.name}
              >
                {typeof tag === "string" ? tag : tag.name}
              </option>
            ))}
          </select>
        </div>

        {filteredArticles.length === 0 ? (
          <p className="text-gray-700 text-base">
            😶 Nothing matched. Try different filters?
          </p>
        ) : (
          <ul className="grid sm:grid-cols-2 lg:grid-cols-3 gap-5">
            {filteredArticles.map((article) => (
              <li
                key={article.id}
                className="bg-gradient-to-br from-white to-yellow-50 p-4 rounded-xl border border-orange-300 shadow-md hover:scale-105 hover:shadow-xl transition-transform"
              >
                <h3 className="text-lg font-bold text-indigo-700 flex items-center gap-2">
                  <FileText className="w-4 h-4" />
                  {article.title}
                </h3>
                <p className="text-base text-gray-700 mt-1">
                  📂 {article.category_name || article.category_id}
                </p>
                <div className="flex flex-wrap gap-2 mt-2">
                  {article.tags?.length > 0 ? (
                    article.tags.map((tag, index) => (
                      <span
                        key={index}
                        className="text-sm bg-orange-100 text-orange-800 px-2 py-0.5 rounded-full"
                      >
                        #{typeof tag === "string" ? tag : tag.name}
                      </span>
                    ))
                  ) : (
                    <span className="text-sm text-gray-500">No tags</span>
                  )}
                </div>
                <button
                  onClick={() =>
                    router.push(`/knowledge-base/view/${article.id}`)
                  }                  
                  className="mt-2 text-base text-blue-600 hover:underline"
                >
                  🚀 Read Article
                </button>
              </li>
            ))}
          </ul>
        )}
      </div>
    </div>
  );    
}
