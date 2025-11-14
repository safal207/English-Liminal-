import { ArrowRight, Star } from 'lucide-react'

export default function Hero() {
  return (
    <section className="relative overflow-hidden bg-gradient-to-b from-blue-50 to-white pt-20 pb-16 sm:pt-32 sm:pb-24">
      {/* Background decoration */}
      <div className="absolute inset-0 -z-10">
        <div className="absolute top-0 right-0 w-96 h-96 bg-blue-100 rounded-full blur-3xl opacity-50"></div>
        <div className="absolute bottom-0 left-0 w-96 h-96 bg-purple-100 rounded-full blur-3xl opacity-50"></div>
      </div>

      <div className="mx-auto max-w-7xl px-6 lg:px-8">
        <div className="mx-auto max-w-2xl text-center">
          {/* Trust indicators */}
          <div className="mb-8 flex items-center justify-center gap-6 text-sm text-gray-600">
            <div className="flex items-center gap-1">
              {[...Array(5)].map((_, i) => (
                <Star key={i} className="w-4 h-4 fill-yellow-400 text-yellow-400" />
              ))}
              <span className="ml-1 font-semibold text-gray-900">4.8</span>
            </div>
            <div className="h-4 w-px bg-gray-300"></div>
            <div>10,000+ learners</div>
            <div className="h-4 w-px bg-gray-300"></div>
            <div>26 scenarios</div>
          </div>

          {/* Main headline */}
          <h1 className="text-5xl font-bold tracking-tight text-gray-900 sm:text-6xl lg:text-7xl">
            Stop Studying English.
            <br />
            <span className="bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
              Start Living It.
            </span>
          </h1>

          {/* Subheadline */}
          <p className="mt-6 text-lg leading-8 text-gray-600 sm:text-xl">
            Master real-world English through immersive scenarios. Practice job interviews,
            startup pitches, sales calls, and business conversations—with AI-powered speech
            recognition that actually works.
          </p>

          {/* CTA buttons */}
          <div className="mt-10 flex items-center justify-center gap-4">
            <a
              href="#pricing"
              className="group inline-flex items-center gap-2 rounded-full bg-blue-600 px-8 py-4 text-sm font-semibold text-white shadow-lg hover:bg-blue-700 transition-all hover:scale-105"
            >
              Start Learning Free
              <ArrowRight className="w-4 h-4 group-hover:translate-x-1 transition-transform" />
            </a>
            <a
              href="#how-it-works"
              className="inline-flex items-center gap-2 rounded-full border-2 border-gray-300 px-8 py-4 text-sm font-semibold text-gray-900 hover:border-gray-400 transition-all"
            >
              See How It Works
            </a>
          </div>

          {/* Social proof tag */}
          <div className="mt-8 flex items-center justify-center gap-2 text-sm text-gray-500">
            <svg className="w-5 h-5 text-green-500" fill="currentColor" viewBox="0 0 20 20">
              <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clipRule="evenodd" />
            </svg>
            Free forever • No credit card required • 3 roles unlocked
          </div>
        </div>

        {/* App preview mockup */}
        <div className="mt-16 sm:mt-24">
          <div className="relative mx-auto max-w-4xl">
            <div className="absolute -inset-4 bg-gradient-to-r from-blue-500 to-purple-500 rounded-3xl blur-2xl opacity-20"></div>
            <div className="relative rounded-2xl bg-gray-900 p-2 shadow-2xl ring-1 ring-gray-900/10">
              <div className="bg-gray-800 rounded-lg p-8 text-center">
                <div className="text-gray-400 text-sm mb-4">App Screenshot Placeholder</div>
                <div className="grid grid-cols-3 gap-4">
                  {['Scenario 1', 'Scenario 2', 'Scenario 3'].map((name) => (
                    <div key={name} className="bg-gray-700 rounded-lg p-4 h-32 flex items-center justify-center text-gray-300 text-sm">
                      {name}
                    </div>
                  ))}
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  )
}
