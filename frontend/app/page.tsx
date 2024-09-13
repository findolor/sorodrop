"use client"

import { useEffect, useRef } from "react"
import * as THREE from "three"
import {
  ArrowUpTrayIcon,
  WrenchScrewdriverIcon,
  RocketLaunchIcon,
  BoltIcon,
  CurrencyDollarIcon,
  TruckIcon,
  AdjustmentsHorizontalIcon,
  SparklesIcon,
  CubeIcon,
  PresentationChartLineIcon,
} from "@heroicons/react/24/solid"
import SocialIcon from "@/components/SocialIcon"

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

export default function Home() {
  const mountRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    if (!mountRef.current) return

    const scene = new THREE.Scene()
    const camera = new THREE.PerspectiveCamera(
      75,
      window.innerWidth / window.innerHeight,
      0.1,
      1000
    )
    camera.position.z = 50

    const renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true })
    renderer.setSize(window.innerWidth, window.innerHeight)
    renderer.setClearColor(0x000000, 0)
    mountRef.current.appendChild(renderer.domElement)

    const particlesCount = 5000
    const particles = new THREE.BufferGeometry()
    const positions = new Float32Array(particlesCount * 3)

    for (let i = 0; i < particlesCount * 3; i += 3) {
      positions[i] = (Math.random() - 0.5) * 100
      positions[i + 1] = Math.random() * 100 - 50
      positions[i + 2] = (Math.random() - 0.5) * 50
    }

    particles.setAttribute("position", new THREE.BufferAttribute(positions, 3))

    const particleMaterial = new THREE.PointsMaterial({
      color: 0x7a53ac,
      size: 0.1,
      transparent: true,
      opacity: 0.6,
    })

    const particleSystem = new THREE.Points(particles, particleMaterial)
    scene.add(particleSystem)

    const animate = () => {
      requestAnimationFrame(animate)

      const positions = particles.attributes.position.array as Float32Array
      for (let i = 0; i < positions.length; i += 3) {
        positions[i + 1] -= 0.05 // Reduced from 0.1 to 0.05 to slow down the particles
        if (positions[i + 1] < -50) positions[i + 1] = 50
      }
      particles.attributes.position.needsUpdate = true

      particleSystem.rotation.y += 0.0003 // Reduced from 0.0005 to 0.0003 to slow down the rotation

      renderer.render(scene, camera)
    }
    animate()

    const handleResize = () => {
      camera.aspect = window.innerWidth / window.innerHeight
      camera.updateProjectionMatrix()
      renderer.setSize(window.innerWidth, window.innerHeight)
    }
    window.addEventListener("resize", handleResize)

    return () => {
      window.removeEventListener("resize", handleResize)
      mountRef.current?.removeChild(renderer.domElement)
    }
  }, [])

  return (
    <div className="relative flex flex-col min-h-screen bg-background-950 text-text-100 overflow-hidden">
      <div className="fixed inset-0 w-full h-full" ref={mountRef}></div>

      <div className="relative z-10 flex-1">
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

        <header className="text-center py-8 sm:py-16 pb-6 sm:pb-10 bg-transparent px-4">
          <h2 className="text-4xl sm:text-6xl font-bold mb-4 text-transparent bg-clip-text bg-gradient-to-r from-orange-500/30 via-[#f2eef7] to-purple-500 animate-gradient">
            Effortless Airdrops, Maximum Reach
          </h2>
          <p className="text-lg sm:text-xl mb-6 sm:mb-8 text-text-200">
            Simplify token and NFT airdrops on Stellar.
          </p>
        </header>

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
                icon={
                  <TruckIcon className="text-white w-5 h-5 sm:w-6 sm:h-6" />
                }
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

        <section className="py-8 sm:py-16 bg-background-955">
          <div className="container mx-auto px-4">
            <h2 className="text-2xl sm:text-3xl font-bold text-center mb-6 sm:mb-10 text-text-50">
              Use Cases
            </h2>
            <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 sm:gap-8">
              <Item
                title="Token Launch"
                description="Easily airdrop to thousands of users at once. Sorodrop allows you to distribute tokens quickly and efficiently, making it ideal for new token launches that need to reach a large audience."
                icon={
                  <SparklesIcon className="text-white w-6 h-6 sm:w-8 sm:h-8" />
                }
              />
              <Item
                title="NFT Distribution"
                description="Distribute NFTs to your community with zero manual effort. Whether you're rewarding loyal followers or releasing a new collection, Sorodrop simplifies NFT airdrops at scale."
                icon={<CubeIcon className="text-white w-6 h-6 sm:w-8 sm:h-8" />}
              />
              <Item
                title="Promotional Airdrops"
                description="Engage users in stages, keeping them active and involved. Sorodrop supports multi-stage airdrops, making it perfect for promotional campaigns that need ongoing engagement."
                icon={
                  <PresentationChartLineIcon className="text-white w-6 h-6 sm:w-8 sm:h-8" />
                }
              />
            </div>
          </div>
        </section>

        <section className="py-8 sm:py-16 text-center px-4">
          <h2 className="text-2xl sm:text-3xl font-bold mb-6 sm:mb-8 text-text-50">
            Ready to simplify your airdrops?
          </h2>
          <div className="flex justify-center space-x-4">
            <SocialIcon type="telegram" />
            <SocialIcon type="github" />
          </div>
        </section>

        <footer className="py-6 sm:py-8 text-center text-text-50">
          <p>&copy; 2024 Sorodrop. All rights reserved.</p>
        </footer>
      </div>
    </div>
  )
}
