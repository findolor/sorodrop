import {
  SparklesIcon,
  CubeIcon,
  PresentationChartLineIcon,
} from "@heroicons/react/24/solid"

type UseCaseItemProps = {
  title: string
  description: string
  icon: JSX.Element
}

const UseCaseItem = ({ title, description, icon }: UseCaseItemProps) => (
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

const UseCases = () => (
  <section className="py-8 sm:py-16 bg-background-955">
    <div className="container mx-auto px-4">
      <h2 className="text-2xl sm:text-3xl font-bold text-center mb-6 sm:mb-10 text-text-50">
        Use Cases
      </h2>
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 sm:gap-8">
        <UseCaseItem
          title="Token Launch"
          description="Easily airdrop to thousands of users at once. Sorodrop allows you to distribute tokens quickly and efficiently, making it ideal for new token launches that need to reach a large audience."
          icon={<SparklesIcon className="text-white w-6 h-6 sm:w-8 sm:h-8" />}
        />
        <UseCaseItem
          title="NFT Distribution"
          description="Distribute NFTs to your community with zero manual effort. Whether you're rewarding loyal followers or releasing a new collection, Sorodrop simplifies NFT airdrops at scale."
          icon={<CubeIcon className="text-white w-6 h-6 sm:w-8 sm:h-8" />}
        />
        <UseCaseItem
          title="Promotional Airdrops"
          description="Engage users in stages, keeping them active and involved. Sorodrop supports multi-stage airdrops, making it perfect for promotional campaigns that need ongoing engagement."
          icon={
            <PresentationChartLineIcon className="text-white w-6 h-6 sm:w-8 sm:h-8" />
          }
        />
      </div>
    </div>
  </section>
)

export default UseCases
