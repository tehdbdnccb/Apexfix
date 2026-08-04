"use client";

import { useState, useEffect } from "react";
import { useRouter } from "next/navigation";
import api from "@/lib/api";
import { useAuthStore } from "@/store/useAuthStore";
import { MapPin, Star, ShieldCheck, Zap, Loader2, AlertCircle } from "lucide-react";

interface RankedTechnician {
  technician: {
    id: string;
    shop_name: string;
    location_name: string;
    certification_level: number;
    part_authenticity_score: number;
    speed_score: number;
    is_verified: boolean;
  };
  distance_km: number;
  final_score: number;
}

export default function MatchPage() {
  const router = useRouter();
  const { isAuthenticated } = useAuthStore();
  
  const [technicians, setTechnicians] = useState<RankedTechnician[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [locationStatus, setLocationStatus] = useState("Locating you...");

  useEffect(() => {
    // Redirect if not authenticated
    if (!isAuthenticated && typeof window !== "undefined") {
      const token = localStorage.getItem("token");
      if (!token) {
        router.push("/login");
        return;
      }
    }

    // Get user's location
    if ("geolocation" in navigator) {
      navigator.geolocation.getCurrentPosition(
        (position) => {
          setLocationStatus("Location found. Searching for technicians...");
          fetchTechnicians(position.coords.latitude, position.coords.longitude);
        },
        (err) => {
          console.error(err);
          setError("Failed to get your location. Please enable location permissions.");
          setLoading(false);
        }
      );
    } else {
      setError("Geolocation is not supported by your browser.");
      setLoading(false);
    }
  }, [isAuthenticated, router]);

  const fetchTechnicians = async (latitude: number, longitude: number) => {
    try {
      const response = await api.get("/technicians/match", {
        params: {
          latitude,
          longitude,
          max_distance_km: 50.0, // Search within 50km
        },
      });
      setTechnicians(response.data);
    } catch (err: any) {
      setError("Failed to fetch technicians. Please try again later.");
    } finally {
      setLoading(false);
    }
  };

  if (loading) {
    return (
      <div className="min-h-[calc(100vh-4rem)] flex flex-col items-center justify-center bg-gray-50">
        <Loader2 className="h-10 w-10 text-blue-600 animate-spin mb-4" />
        <p className="text-gray-600 font-medium">{locationStatus}</p>
      </div>
    );
  }

  return (
    <div className="min-h-[calc(100vh-4rem)] bg-gray-50 py-10 px-4 sm:px-6 lg:px-8">
      <div className="max-w-5xl mx-auto">
        <div className="mb-8">
          <h1 className="text-3xl font-extrabold text-gray-900">Recommended Technicians</h1>
          <p className="mt-2 text-gray-600">
            Ranked by proximity, quality, and speed using our intelligent matching algorithm.
          </p>
        </div>

        {error && (
          <div className="mb-8 p-4 bg-red-50 border border-red-200 rounded-md flex items-center text-red-700">
            <AlertCircle className="h-5 w-5 mr-2" />
            {error}
          </div>
        )}

        {!error && technicians.length === 0 && (
          <div className="text-center py-12 bg-white rounded-xl shadow-sm border border-gray-100">
            <MapPin className="mx-auto h-12 w-12 text-gray-400 mb-4" />
            <h3 className="text-lg font-medium text-gray-900">No technicians found</h3>
            <p className="mt-1 text-gray-500">We couldn't find any repair shops within 50km of your location.</p>
          </div>
        )}

        <div className="space-y-6">
          {technicians.map((match, index) => (
            <div 
              key={match.technician.id} 
              className="bg-white p-6 rounded-xl shadow-sm border border-gray-100 hover:shadow-md transition-shadow flex flex-col md:flex-row md:items-center justify-between"
            >
              <div className="flex-1">
                <div className="flex items-center space-x-3 mb-2">
                  <h2 className="text-xl font-bold text-gray-900">
                    {match.technician.shop_name}
                  </h2>
                  {index === 0 && (
                    <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800">
                      Top Match
                    </span>
                  )}
                  {match.technician.is_verified && (
                    <span title="Verified Partner" className="inline-flex items-center">
                      <ShieldCheck className="h-5 w-5 text-blue-600" />
                    </span>
                  )}
                </div>
                
                <div className="flex items-center text-gray-500 text-sm mb-4">
                  <MapPin className="h-4 w-4 mr-1" />
                  {match.technician.location_name} • {match.distance_km.toFixed(1)} km away
                </div>

                <div className="flex flex-wrap gap-4 text-sm">
                  <div className="flex items-center text-gray-700">
                    <Star className="h-4 w-4 text-yellow-400 mr-1.5" fill="currentColor" />
                    <span>Score: <span className="font-semibold text-gray-900">{(match.final_score * 10).toFixed(1)}/10</span></span>
                  </div>
                  <div className="flex items-center text-gray-700">
                    <ShieldCheck className="h-4 w-4 text-gray-400 mr-1.5" />
                    <span>Quality: {(match.technician.part_authenticity_score * 100).toFixed(0)}%</span>
                  </div>
                  <div className="flex items-center text-gray-700">
                    <Zap className="h-4 w-4 text-gray-400 mr-1.5" />
                    <span>Speed: {(match.technician.speed_score * 100).toFixed(0)}%</span>
                  </div>
                </div>
              </div>

              <div className="mt-6 md:mt-0 md:ml-6 flex md:flex-col justify-end space-y-0 md:space-y-3 space-x-3 md:space-x-0">
                <button 
                  onClick={() => alert("Booking flow would start here!")}
                  className="w-full md:w-auto inline-flex justify-center items-center px-6 py-2.5 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 transition-colors"
                >
                  Book Repair
                </button>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}