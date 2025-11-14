import { X, Check, BookOpen, Mic, Users, Brain } from 'lucide-react'

export default function ProblemSolution() {
  const problems = [
    {
      icon: X,
      title: 'Grammar drills that lead nowhere',
      description: 'Conjugating verbs for hours but freezing when someone asks, "How are you?"',
    },
    {
      icon: X,
      title: 'Textbook English ≠ Real English',
      description: 'Your teacher never taught you how to pitch investors or negotiate a raise.',
    },
    {
      icon: X,
      title: 'One-size-fits-all courses',
      description: 'Generic lessons about airports and hotels when you need to lead standup meetings.',
    },
  ]

  const solutions = [
    {
      icon: Users,
      title: 'Real Situations You'll Face Tomorrow',
      description: 'Practice the exact scenarios you need: job interviews, code reviews, investor pitches, visa applications. 26 scenarios across 7 professional roles.',
    },
    {
      icon: Mic,
      title: 'Speak, Don't Memorize',
      description: 'AI-powered speech recognition gives instant feedback on pronunciation and fluency. No awkward tutors. No scheduling. Just you and real conversations.',
    },
    {
      icon: BookOpen,
      title: 'Learn in Context, Not Lists',
      description: 'Every phrase comes from a real scenario. See how native speakers actually use "I was wondering if..." in a salary negotiation.',
    },
    {
      icon: Brain,
      title: 'Remember Forever with Spaced Repetition',
      description: 'Our algorithm knows when you'll forget. Review scenarios at the perfect time to move knowledge into long-term memory.',
    },
  ]

  return (
    <div className="bg-white py-24 sm:py-32">
      <div className="mx-auto max-w-7xl px-6 lg:px-8">
        {/* Problem Section */}
        <div className="mx-auto max-w-2xl text-center mb-16">
          <h2 className="text-base font-semibold leading-7 text-blue-600">The Problem</h2>
          <p className="mt-2 text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
            Why Traditional English Learning Fails
          </p>
          <p className="mt-6 text-lg leading-8 text-gray-600">
            You've tried apps, courses, and tutors. You know the grammar. But when it's time to speak...
          </p>
        </div>

        <div className="mx-auto grid max-w-2xl grid-cols-1 gap-8 lg:max-w-none lg:grid-cols-3 mb-24">
          {problems.map((problem) => (
            <div key={problem.title} className="bg-red-50 rounded-2xl p-8 border-2 border-red-100">
              <problem.icon className="h-8 w-8 text-red-600 mb-4" />
              <h3 className="text-xl font-semibold text-gray-900 mb-2">{problem.title}</h3>
              <p className="text-gray-600">{problem.description}</p>
            </div>
          ))}
        </div>

        {/* Solution Section */}
        <div className="mx-auto max-w-2xl text-center mb-16">
          <h2 className="text-base font-semibold leading-7 text-blue-600">The Solution</h2>
          <p className="mt-2 text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
            How English Liminal Works
          </p>
          <p className="mt-6 text-lg leading-8 text-gray-600">
            Stop studying English. Start <em>living</em> it.
          </p>
        </div>

        <div className="mx-auto grid max-w-2xl grid-cols-1 gap-8 lg:max-w-none lg:grid-cols-2">
          {solutions.map((solution) => (
            <div key={solution.title} className="bg-blue-50 rounded-2xl p-8 border-2 border-blue-100">
              <solution.icon className="h-8 w-8 text-blue-600 mb-4" />
              <h3 className="text-xl font-semibold text-gray-900 mb-2">{solution.title}</h3>
              <p className="text-gray-600">{solution.description}</p>
            </div>
          ))}
        </div>
      </div>
    </div>
  )
}
