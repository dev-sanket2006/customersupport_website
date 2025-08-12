"use client";

import { useEffect, useState } from "react";
import { useParams } from "next/navigation";
import { BadgeCheck, XCircle, Tag as TagIcon } from "lucide-react";

export default function ViewArticlePage() {
  const { id } = useParams();
  const [article, setArticle] = useState(null);
  const [categories, setCategories] = useState([]);
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (!id) return;

    const fetchData = async () => {
      try {
        const [articleRes, categoriesRes] = await Promise.all([
          fetch(`http://localhost:8000/kb/articles/${id}`),
          fetch("http://localhost:8000/kb/categories"),
        ]);

        if (!articleRes.ok) throw new Error("Failed to load article");
        if (!categoriesRes.ok) throw new Error("Failed to load categories");

        const articleData = await articleRes.json();
        const categoriesData = await categoriesRes.json();

        setArticle(articleData);
        setCategories(categoriesData);
      } catch (err) {
        console.error(err);
        setError("Failed to load article or categories");
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, [id]);

  if (loading)
    return (
      <p className="p-6 text-center text-purple-600 font-semibold animate-pulse">
        Loading your awesome article...
      </p>
    );

  if (error)
    return (
      <p className="p-6 text-center text-red-600 bg-red-100 rounded-md font-semibold">
        {error}
      </p>
    );

  const categoryName =
    categories.find((c) => c.id === article.category_id)?.name || "Unknown";

  return (
    <div className="min-h-screen bg-gradient-to-br from-purple-100 via-white to-blue-100 py-12 px-4">
      <div className="max-w-5xl mx-auto shadow-xl rounded-3xl overflow-hidden border border-gray-200">
        {/* Header */}
        <div className="bg-gradient-to-r from-purple-700 to-indigo-700 p-8 md:p-10 text-white">
          <h1 className="text-4xl md:text-5xl font-bold mb-4 leading-tight">
            {article.title}
          </h1>
          <div className="flex flex-wrap gap-3 text-sm font-medium">
            <span className="bg-white/20 px-4 py-1 rounded-full flex items-center gap-2 hover:bg-white/30 transition">
              <BadgeCheck className="w-4 h-4" />
              Category: {categoryName}
            </span>
            <span
              className={`px-4 py-1 rounded-full flex items-center gap-2 ${
                article.is_published
                  ? "bg-green-500/20 text-green-100"
                  : "bg-yellow-500/20 text-yellow-100"
              }`}
            >
              {article.is_published ? (
                <>
                  <BadgeCheck className="w-4 h-4" />
                  Published
                </>
              ) : (
                <>
                  <XCircle className="w-4 h-4" />
                  Not Published
                </>
              )}
            </span>

            {/* ✅ Tags */}
            {article.tags && article.tags.length > 0 && (
              <div className="flex flex-wrap items-center gap-2">
                {article.tags.map((tag, index) => (
                  <span
                    key={index}
                    className="bg-indigo-200/40 text-white px-3 py-1 rounded-full flex items-center gap-1 text-xs font-medium border border-white/30"
                  >
                    <TagIcon className="w-3 h-3" />
                    {tag}
                  </span>
                ))}
              </div>
            )}
          </div>
        </div>

        {/* Content */}
        <div className="relative bg-white/70 backdrop-blur-lg bg-gradient-to-br from-white/80 via-gray-100/80 to-blue-50/80 p-8 md:p-10 text-gray-800 border-t border-gray-200 shadow-inner">
          <div className="overflow-y-auto max-h-[75vh] custom-scrollbar">
            <article className="prose prose-lg max-w-none prose-indigo leading-relaxed tracking-wide">
              {article.content}
            </article>
          </div>
        </div>
      </div>
    </div>
  );
}
