import type { Metadata } from "next"
import "./globals.css"

export const metadata: Metadata = {
  title: "Sorodrop",
  description:
    "Open-source tool for efficient Stellar airdrops enabling scalable and cost-effective token & NFT distribution via smart contracts.",
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en" className="light">
      <body className={`font-montserrat antialiased`}>{children}</body>
    </html>
  )
}
