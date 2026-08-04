import Link from "next/link";
import { ShieldCheck, Zap, MapPin, Wrench } from "lucide-react";

export default function Home() {
  return (
    <div className="flex flex-col items-center justify-center pt-20 pb-16 px-4 sm:px-6 lg:px-8 text-center">
      {/* Hero Section */}
      <div className="max-w-3xl space-y-8">
        <div className="inline-flex items-center justify-center p-3 bg-blue-100 rounded-full mb-4">
          <Wrench className="h-8 w-8 text-blue-600" />
        </div>
        <h1 className="text-5xl font-extrabold text-gray-900 tracking-tight sm:text-6xl">
          Find the Right Technician, <span className="text-blue-600">Right Now.</span>
        </h1>
        <p className="mt-6 text-xl text-gray-500 max-w-2xl mx-auto">
          Our smart matching algorithm connects you with trusted repair experts in your area based on certification, part authenticity, speed, and proximity.
        </p>
        
        <div className="mt-10 flex flex-col sm:flex-row gap-4 justify-center">
          <Link 
            href="/match" 
            className="inline-flex justify-center items-center px-8 py-3 border border-transparent text-base font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 md:py-4 md:text-lg md:px-10 shadow-sm transition-all"
          >
            Find a Technician
          </Link>
          <Link 
            href="/register" 
            className="inline-flex justify-center items-center px-8 py-3 border border-gray-300 text-base font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 md:py-4 md:text-lg md:px-10 shadow-sm transition-all"
          >
            Become a Partner
          </Link>
        </div>
      </div>

      {/* Feature Section */}
      <div className="mt-24 max-w-7xl mx-auto grid grid-cols-1 gap-12 sm:grid-cols-3">
        <div className="flex flex-col items-center text-center">
          <div className="flex items-center justify-center h-16 w-16 rounded-full bg-green-100 text-green-600 mb-6">
            <ShieldCheck className="h-8 w-8" />
          </div>
          <h3 className="text-xl font-bold text-gray-900">Verified Quality</h3>
          <p className="mt-2 text-base text-gray-500">
            We rank technicians by their certification levels and their track record for using authentic parts.
          </p>
        </div>

        <div className="flex flex-col items-center text-center">
          <div className="flex items-center justify-center h-16 w-16 rounded-full bg-yellow-100 text-yellow-600 mb-6">
            <Zap className="h-8 w-8" />
          </div>
          <h3 className="text-xl font-bold text-gray-900">Fast Turnaround</h3>
          <p className="mt-2 text-base text-gray-500">
            Need it fixed yesterday? Our algorithm factors in the historical speed scores of our repair shops.
          </p>
        </div>

        <div className="flex flex-col items-center text-center">
          <div className="flex items-center justify-center h-16 w-16 rounded-full bg-red-100 text-red-600 mb-6">
            <MapPin className="h-8 w-8" />
          </div>
          <h3 className="text-xl font-bold text-gray-900">Hyper-Local</h3>
          <p className="mt-2 text-base text-gray-500">
            Powered by PostGIS, we calculate real-time spatial distances so you never have to travel far.
          </p>
        </div>
      </div>
    </div>
  );
}