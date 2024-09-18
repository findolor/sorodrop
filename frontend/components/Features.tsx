import {
  ArrowUpTrayIcon,
  WrenchScrewdriverIcon,
  RocketLaunchIcon,
} from "@heroicons/react/24/solid"

type ItemProps = {
  title: string
  description: string
  icon: JSX.Element
}

const Item = ({ title, description, icon }: ItemProps) => (
  <div className="bg-transparent backdrop-blur-sm bg-opacity-5 p-4 sm:p-6 rounded-xl border border-background-700 hover:border-orange-300/10 transition duration-300 ease-in-out transform hover:-translate-y-1 shadow-[0_0_7px_rgba(255,173,102,0.15)] hover:shadow-[0_0_7px_rgba(255,173,102,0.3)]">
    <div className="flex items-center mb-3 sm:mb-4">
      <div className="bg-accent-500 p-2 sm:p-3 rounded-full mr-3 sm:mr-4">
        {icon}
      </div>
      <h3 className="text-lg sm:text-xl font-semibold text-text-50">{title}</h3>
    </div>
    <p className="text-sm sm:text-base text-text-200">{description}</p>
  </div>
)

const Features = () => (
  <section className="py-8 sm:py-16">
    <div className="container mx-auto px-4">
      <h2 className="text-2xl sm:text-3xl font-bold text-center mb-6 sm:mb-10 text-text-50">
        How It Works
      </h2>
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 sm:gap-8">
        <Item
          title="Upload Your Airdrop List"
          icon={
            <ArrowUpTrayIcon className="text-white w-6 h-6 sm:w-8 sm:h-8" />
          }
          description="Easily upload your airdrop recipients using a CSV file and avoid manual entry mistakes."
        />
        <Item
          title="Set Airdrop Details"
          icon={
            <WrenchScrewdriverIcon className="text-white w-6 h-6 sm:w-8 sm:h-8" />
          }
          description="Customize your airdrop by adjusting start time, token amounts, and more for full control."
        />
        <Item
          title="Reach Your Audience"
          icon={
            <RocketLaunchIcon className="text-white w-6 h-6 sm:w-8 sm:h-8" />
          }
          description="Go live with your airdrop in just one click and watch your tokens or NFTs reach thousands instantly."
        />
      </div>
    </div>
  </section>
)

export default Features
