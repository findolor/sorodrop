import SocialIcon from "@/components/SocialIcon"
import Link from "next/link"

const Header = () => (
  <header>
    <div className="flex flex-col sm:flex-row justify-between items-center py-4 sm:py-6 px-4 sm:px-10">
      <h1 className="text-3xl sm:text-5xl font-medium text-text-50 mb-4 sm:mb-0">
        Sorodrop
      </h1>
      <div className="flex items-center space-x-4">
        <SocialIcon type="telegram" />
        <SocialIcon type="github" />
        <Link href="/app" className="ml-4">
          <button className="bg-accent-500 hover:bg-accent-600 text-white font-bold py-2 px-4 rounded transition duration-300">
            Open App
          </button>
        </Link>
      </div>
    </div>
    <div className="w-full h-px bg-gradient-to-r from-transparent via-text-300 to-transparent opacity-50"></div>
  </header>
)

export default Header
