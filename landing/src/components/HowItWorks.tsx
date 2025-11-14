import { Headphones, Mic, Eye, Target } from 'lucide-react'

export default function HowItWorks() {
  const steps = [
    {
      number: '01',
      icon: Headphones,
      title: 'Listen',
      description: 'Hear how native speakers handle real situations. Notice their phrasing, tone, and rhythm.',
      color: 'from-blue-500 to-blue-600',
    },
    {
      number: '02',
      icon: Mic,
      title: 'Speak',
      description: 'Practice the same scenario yourself. Our AI listens and gives instant feedback on pronunciation.',
      color: 'from-purple-500 to-purple-600',
    },
    {
      number: '03',
      icon: Eye,
      title: 'Contrast',
      description: 'Compare your version to the native speaker. See what you missed and why it matters.',
      color: 'from-pink-500 to-pink-600',
    },
    {
      number: '04',
      icon: Target,
      title: 'Apply',
      description: 'Use it in real life. Then review with spaced repetition so it sticks forever.',
      color: 'from-orange-500 to-orange-600',
    },
  ]

  return (
    <section id="how-it-works" className="bg-white py-24 sm:py-32">
      <div className="mx-auto max-w-7xl px-6 lg:px-8">
        <div className="mx-auto max-w-2xl text-center">
          <h2 className="text-base font-semibold leading-7 text-blue-600">How It Works</h2>
          <p className="mt-2 text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
            4 Steps to Fluency
          </p>
          <p className="mt-6 text-lg leading-8 text-gray-600">
            Our proven learning method moves knowledge from short-term to long-term memory.
          </p>
        </div>

        <div className="mx-auto mt-16 max-w-5xl">
          <div className="grid grid-cols-1 gap-8 lg:grid-cols-4">
            {steps.map((step, index) => (
              <div key={step.number} className="relative">
                {/* Connector line */}
                {index < steps.length - 1 && (
                  <div className="absolute top-12 left-1/2 hidden h-px w-full bg-gradient-to-r from-gray-300 to-transparent lg:block"></div>
                )}

                {/* Step card */}
                <div className="relative text-center">
                  {/* Number badge */}
                  <div className={`mx-auto mb-4 inline-flex h-16 w-16 items-center justify-center rounded-full bg-gradient-to-br ${step.color} text-white text-xl font-bold shadow-lg`}>
                    {step.number}
                  </div>

                  {/* Icon */}
                  <div className="mx-auto mb-4 inline-flex h-12 w-12 items-center justify-center rounded-full bg-gray-100">
                    <step.icon className="h-6 w-6 text-gray-700" />
                  </div>

                  {/* Title */}
                  <h3 className="text-xl font-semibold text-gray-900 mb-2">{step.title}</h3>

                  {/* Description */}
                  <p className="text-sm text-gray-600 leading-relaxed">{step.description}</p>
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* CTA */}
        <div className="mt-16 text-center">
          <p className="text-lg text-gray-600 mb-6">
            Ready to stop studying and start living English?
          </p>
          <a
            href="#pricing"
            className="inline-flex items-center gap-2 rounded-full bg-blue-600 px-8 py-4 text-sm font-semibold text-white shadow-lg hover:bg-blue-700 transition-all hover:scale-105"
          >
            Try It Free
          </a>
        </div>
      </div>
    </section>
  )
}
