import SocialIcon from "@/components/SocialIcon"

const Header = () => (
  <header>
    <div className="flex flex-col sm:flex-row justify-between items-center py-4 sm:py-8 px-4 sm:px-10">
      <h1 className="text-3xl sm:text-5xl font-medium text-text-50 mb-4 sm:mb-0">
        Sorodrop
      </h1>
      <div className="flex space-x-4">
        <SocialIcon type="telegram" />
        <SocialIcon type="github" />
      </div>
    </div>
    <div className="w-full h-px bg-gradient-to-r from-transparent via-text-300 to-transparent my-4 sm:my-6 mt-0 opacity-50"></div>
  </header>
)

export default Header
