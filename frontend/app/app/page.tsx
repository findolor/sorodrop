import Link from "next/link"

export default function ComingSoon() {
  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-background-950 text-text-100 p-4">
      <h1 className="text-4xl sm:text-6xl font-bold mb-4 text-transparent bg-clip-text bg-gradient-to-r from-orange-500/30 via-[#f2eef7] to-purple-500 leading-tight py-2 text-center">
        Coming Soon
      </h1>
      <p className="text-lg sm:text-xl mb-8 text-text-200 text-center">
        We&apos;re working hard to bring you an amazing airdrop experience!
      </p>
      <div className="animate-bounce flex justify-center w-full">
        <svg
          className="w-6 h-6 text-accent-500"
          fill="none"
          strokeLinecap="round"
          strokeLinejoin="round"
          strokeWidth="2"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path d="M19 14l-7 7m0 0l-7-7m7 7V3"></path>
        </svg>
      </div>
      <Link href="/" className="mt-8">
        <button className="bg-accent-500 hover:bg-accent-600 text-white font-bold py-2 px-4 rounded transition duration-300">
          Back to Home
        </button>
      </Link>
    </div>
  )
}
