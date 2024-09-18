"use client"

import Header from "@/components/Header"
import Hero from "@/components/Hero"
import Features from "@/components/Features"
import Footer from "@/components/Footer"
import Benefits from "@/components/Benefits"
import UseCases from "@/components/UseCases"
import useBackgroundAnimation from "@/hooks/useBackgroundAnimation"

export default function Home() {
  const backgroundRef = useBackgroundAnimation()

  return (
    <div className="relative flex flex-col min-h-screen bg-background-950 text-text-100 overflow-hidden">
      <div className="fixed inset-0 w-full h-full" ref={backgroundRef}></div>

      <div className="relative z-10 flex-1">
        <Header />
        <Hero />
        <Features />
        <Benefits />
        <UseCases />
        <Footer />
      </div>
    </div>
  )
}
