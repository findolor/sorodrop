import SocialIcon from "@/components/SocialIcon"
import Link from "next/link"
import Image from "next/image"
// import Logo from "@/images/logo.svg"
import LogoWithText from "@/images/logo-with-text.svg"
import { Button } from "@/components/ui/button"

const Header = () => (
  <header>
    <div className="flex flex-col sm:flex-row justify-between items-center py-4 sm:py-6 px-4 sm:px-10">
      <div className="flex items-center mb-4 sm:mb-0">
        <Link href="/">
          <Image
            src={LogoWithText}
            alt="Sorodrop Logo"
            width={200}
            height={200}
            className="mr-4"
          />
        </Link>
        {/* <h1 className="text-3xl sm:text-5xl font-medium text-text-50">
          Sorodrop
        </h1> */}
      </div>
      <div className="flex items-center space-x-4">
        <SocialIcon type="telegram" />
        <SocialIcon type="github" />
        <Link href="/app" className="ml-4">
          <Button className="bg-accent-500 hover:bg-accent-600 text-white font-bold py-2 px-4 rounded transition duration-300">
            Open App
          </Button>
        </Link>
      </div>
    </div>
    <div className="w-full h-px bg-gradient-to-r from-transparent via-text-300 to-transparent opacity-50"></div>
  </header>
)

export default Header
