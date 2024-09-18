import {
  BoltIcon,
  CurrencyDollarIcon,
  TruckIcon,
  AdjustmentsHorizontalIcon,
} from "@heroicons/react/24/solid"

type BenefitItemProps = {
  title: string
  description: string
  icon: JSX.Element
}

const BenefitItem = ({ title, description, icon }: BenefitItemProps) => (
  <div className="relative bg-background-800 bg-opacity-50 backdrop-blur-sm p-4 sm:p-6 rounded-lg hover:bg-background-700 hover:bg-opacity-50 transition duration-300 ease-in-out group">
    <div className="absolute inset-0 rounded-lg overflow-hidden">
      <div className="absolute inset-0 bg-gradient-to-br from-yellow-300/20 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300 ease-in-out"></div>
    </div>
    <div className="relative z-10">
      <div className="flex items-center mb-3 sm:mb-4">
        <div className="bg-accent-500 p-1.5 sm:p-2 rounded-full mr-3 sm:mr-4">
          {icon}
        </div>
        <h3 className="text-lg sm:text-xl font-semibold text-text-50">
          {title}
        </h3>
      </div>
      <p className="text-sm sm:text-base text-text-200">{description}</p>
    </div>
  </div>
)

const Benefits = () => (
  <section className="py-8 sm:py-16">
    <div className="container mx-auto px-4">
      <h2 className="text-2xl sm:text-3xl font-bold text-center mb-6 sm:mb-10 text-text-50">
        Benefits
      </h2>
      <div className="grid grid-cols-1 sm:grid-cols-2 gap-6 sm:gap-8">
        <BenefitItem
          title="Instant Setup"
          description="Get your airdrop ready in minutes. With Sorodrop, you don't need technical expertise—just upload your recipient list and you're ready to go."
          icon={<BoltIcon className="text-white w-5 h-5 sm:w-6 sm:h-6" />}
        />
        <BenefitItem
          title="Low Fees"
          description="Minimize gas costs with our efficient system. Sorodrop leverages Stellar's low transaction fees, so you can distribute tokens or NFTs without worrying about high operational costs."
          icon={
            <CurrencyDollarIcon className="text-white w-5 h-5 sm:w-6 sm:h-6" />
          }
        />
        <BenefitItem
          title="Automated Delivery"
          description="Once your airdrop is configured, Sorodrop automatically distributes tokens or NFTs to your recipients without any manual intervention."
          icon={<TruckIcon className="text-white w-5 h-5 sm:w-6 sm:h-6" />}
        />
        <BenefitItem
          title="Customizable"
          description="Adjust your settings anytime. Sorodrop gives you full flexibility to pause, resume, or modify your airdrop even after launch."
          icon={
            <AdjustmentsHorizontalIcon className="text-white w-5 h-5 sm:w-6 sm:h-6" />
          }
        />
      </div>
    </div>
  </section>
)

export default Benefits
