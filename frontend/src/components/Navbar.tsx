"use client";

import Link from "next/link";
import { useAuthStore } from "@/store/useAuthStore";
import { Wrench, User, LogOut } from "lucide-react";

export default function Navbar() {
  const { user, isAuthenticated, logout } = useAuthStore();

  return (
    <nav className="bg-white shadow-sm border-b border-gray-200">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex justify-between h-16 items-center">
          {/* Logo / Brand */}
          <Link href="/" className="flex items-center space-x-2">
            <Wrench className="h-6 w-6 text-blue-600" />
            <span className="font-bold text-xl text-gray-900">RepairMatch</span>
          </Link>

          {/* Navigation Links & Auth */}
          <div className="flex items-center space-x-4">
            {isAuthenticated ? (
              <>
                <Link
                  href="/match"
                  className="text-gray-600 hover:text-blue-600 px-3 py-2 rounded-md text-sm font-medium"
                >
                  Find Technician
                </Link>
                <div className="flex items-center space-x-3 ml-4 pl-4 border-l border-gray-200">
                  <div className="flex items-center text-sm font-medium text-gray-700">
                    <User className="h-4 w-4 mr-1" />
                    {user?.full_name}
                  </div>
                  <button
                    onClick={logout}
                    className="flex items-center text-gray-500 hover:text-red-600 px-2 py-2 rounded-md text-sm font-medium transition-colors"
                  >
                    <LogOut className="h-4 w-4" />
                  </button>
                </div>
              </>
            ) : (
              <>
                <Link
                  href="/login"
                  className="text-gray-600 hover:text-blue-600 px-3 py-2 rounded-md text-sm font-medium"
                >
                  Log in
                </Link>
                <Link
                  href="/register"
                  className="bg-blue-600 text-white hover:bg-blue-700 px-4 py-2 rounded-md text-sm font-medium transition-colors"
                >
                  Sign up
                </Link>
              </>
            )}
          </div>
        </div>
      </div>
    </nav>
  );
}