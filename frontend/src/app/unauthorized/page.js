"use client";

import { useAuth } from "@/context/auth-context";
import { useRouter } from "next/navigation";

export default function UnauthorizedPage() {
  const { user } = useAuth();
  const router = useRouter();

  const handleBack = () => {
    if (user?.role === "admin") {
      router.push("/admin-dashboard");
    } else if (user?.role === "agent") {
      router.push("/agent-dashboard");
    } else {
      router.push("/dashboard");
    }
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-gradient-to-r from-gray-100 to-gray-200 px-4">
      <div className="bg-white shadow-xl rounded-2xl p-10 max-w-lg w-full text-center">
        <div className="text-red-600 text-5xl font-bold mb-4">403</div>
        <h1 className="text-2xl font-semibold text-gray-800 mb-2">
          Unauthorized Access
        </h1>
        <p className="text-gray-600 mb-6">
          {
            "You don't have permission to view this page. Please go back to your dashboard."
          }
        </p>

        <button
          onClick={handleBack}
          className="px-6 py-2 bg-indigo-600 hover:bg-indigo-700 text-white font-medium rounded-lg shadow-sm transition duration-200"
        >
          Go to Dashboard
        </button>
      </div>
    </div>
  );
}
