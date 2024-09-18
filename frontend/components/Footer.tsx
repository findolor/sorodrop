import SocialIcon from "@/components/SocialIcon"

const Footer = () => (
  <footer>
    <section className="py-8 sm:py-16 text-center px-4">
      <h2 className="text-2xl sm:text-3xl font-bold mb-6 sm:mb-8 text-text-50">
        Ready to simplify your airdrops?
      </h2>
      <div className="flex justify-center space-x-4">
        <SocialIcon type="telegram" />
        <SocialIcon type="github" />
      </div>
    </section>
    <div className="py-6 sm:py-8 text-center text-text-50">
      <p>&copy; 2024 Sorodrop. All rights reserved.</p>
    </div>
  </footer>
)

export default Footer
