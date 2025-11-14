import { Code, Plane, Globe, Users as UsersIcon, Lightbulb, TrendingUp, Lock } from 'lucide-react'

export default function Roles() {
  const roles = [
    {
      id: 'qa_abroad',
      icon: Code,
      title: 'QA Engineer Abroad',
      difficulty: 'Beginner',
      scenarios: 4,
      isFree: true,
      description: 'Master technical discussions, bug reports, and team communication in English.',
      scenariosList: [
        'Daily Standup Meeting',
        'Bug Report Discussion',
        'Code Review Feedback',
        'Client Demo Prep',
      ],
    },
    {
      id: 'visa_journey',
      icon: Plane,
      title: 'Visa Journey',
      difficulty: 'Beginner',
      scenarios: 3,
      isFree: true,
      description: 'Ace your visa interview with confidence. Practice common questions and answers.',
      scenariosList: [
        'Visa Interview Preparation',
        'Embassy Questions',
        'Explaining Your Purpose',
      ],
    },
    {
      id: 'remote_developer',
      icon: Code,
      title: 'Remote Developer',
      difficulty: 'Beginner',
      scenarios: 4,
      isFree: true,
      description: 'Navigate async communication, code reviews, and remote pair programming.',
      scenariosList: [
        'Async Standup Updates',
        'Code Review Comments',
        'Pair Programming Session',
        'Tech Debt Discussion',
      ],
    },
    {
      id: 'global_citizen',
      icon: Globe,
      title: 'Global Citizen',
      difficulty: 'Intermediate',
      scenarios: 4,
      isFree: false,
      description: 'Navigate cultural differences, networking events, and expat life conversations.',
      scenariosList: [
        'Networking at International Events',
        'Cultural Small Talk',
        'Expat Community Building',
        'Cross-cultural Negotiations',
      ],
    },
    {
      id: 'family_abroad',
      icon: UsersIcon,
      title: 'Family Abroad',
      difficulty: 'Intermediate',
      scenarios: 2,
      isFree: false,
      description: 'Handle parent-teacher meetings, doctor visits, and family life in English.',
      scenariosList: [
        'Parent-Teacher Conference',
        'Pediatrician Visit',
      ],
    },
    {
      id: 'tech_startup_founder',
      icon: Lightbulb,
      title: 'Tech Startup Founder',
      difficulty: 'Advanced',
      scenarios: 5,
      isFree: false,
      description: 'Pitch investors, lead your team, and network at tech events like a pro.',
      scenariosList: [
        'The 3-Minute Investor Pitch',
        'Customer Discovery Call',
        'Leading Team Through Uncertainty',
        'Networking at TechCrunch',
        'Announcing a Pivot',
      ],
    },
    {
      id: 'sales_professional',
      icon: TrendingUp,
      title: 'Sales Professional',
      difficulty: 'Intermediate',
      scenarios: 4,
      isFree: false,
      description: 'Master discovery calls, demos, objection handling, and closing deals.',
      scenariosList: [
        'SPIN Selling Discovery Call',
        'Product Demo Presentation',
        'Handling Price Objections',
        'Negotiating the Close',
      ],
    },
  ]

  return (
    <section id="roles" className="bg-gray-50 py-24 sm:py-32">
      <div className="mx-auto max-w-7xl px-6 lg:px-8">
        <div className="mx-auto max-w-2xl text-center">
          <h2 className="text-base font-semibold leading-7 text-blue-600">Choose Your Role</h2>
          <p className="mt-2 text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
            7 Roles. 26 Real-World Scenarios.
          </p>
          <p className="mt-6 text-lg leading-8 text-gray-600">
            Learn the English you actually need. Pick your role and practice scenarios you'll face tomorrow.
          </p>
        </div>

        <div className="mx-auto mt-16 grid max-w-2xl grid-cols-1 gap-6 sm:mt-20 lg:max-w-none lg:grid-cols-3">
          {roles.map((role) => (
            <div
              key={role.id}
              className={`relative rounded-2xl p-8 ${
                role.isFree
                  ? 'bg-white border-2 border-gray-200'
                  : 'bg-gradient-to-br from-blue-50 to-purple-50 border-2 border-blue-200'
              } shadow-sm hover:shadow-lg transition-shadow`}
            >
              {/* Free/Premium Badge */}
              <div className="absolute top-4 right-4">
                {role.isFree ? (
                  <span className="inline-flex items-center rounded-full bg-green-100 px-3 py-1 text-xs font-semibold text-green-700">
                    Free
                  </span>
                ) : (
                  <span className="inline-flex items-center gap-1 rounded-full bg-gradient-to-r from-blue-600 to-purple-600 px-3 py-1 text-xs font-semibold text-white">
                    <Lock className="w-3 h-3" />
                    Premium
                  </span>
                )}
              </div>

              {/* Icon */}
              <div className={`inline-flex rounded-lg p-3 ${role.isFree ? 'bg-blue-50' : 'bg-white'}`}>
                <role.icon className={`h-6 w-6 ${role.isFree ? 'text-blue-600' : 'text-purple-600'}`} />
              </div>

              {/* Title & Meta */}
              <h3 className="mt-4 text-xl font-semibold text-gray-900">{role.title}</h3>
              <div className="mt-2 flex items-center gap-3 text-sm text-gray-600">
                <span className="inline-flex items-center gap-1">
                  <span className={`h-2 w-2 rounded-full ${
                    role.difficulty === 'Beginner' ? 'bg-green-500' :
                    role.difficulty === 'Intermediate' ? 'bg-yellow-500' :
                    'bg-red-500'
                  }`}></span>
                  {role.difficulty}
                </span>
                <span>•</span>
                <span>{role.scenarios} scenarios</span>
              </div>

              {/* Description */}
              <p className="mt-4 text-sm text-gray-600">{role.description}</p>

              {/* Scenarios List */}
              <div className="mt-6">
                <p className="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-3">
                  What You'll Practice:
                </p>
                <ul className="space-y-2">
                  {role.scenariosList.map((scenario, idx) => (
                    <li key={idx} className="flex items-start gap-2 text-sm text-gray-700">
                      <svg className="w-4 h-4 text-blue-600 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                        <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clipRule="evenodd" />
                      </svg>
                      <span>{scenario}</span>
                    </li>
                  ))}
                </ul>
              </div>
            </div>
          ))}
        </div>

        {/* Free vs Premium CTA */}
        <div className="mt-16 text-center">
          <p className="text-lg text-gray-600 mb-6">
            Start with <span className="font-semibold text-green-600">3 free roles</span> (12 scenarios).
            Unlock <span className="font-semibold text-blue-600">4 premium roles</span> (14 scenarios) anytime.
          </p>
          <a
            href="#pricing"
            className="inline-flex items-center gap-2 rounded-full bg-blue-600 px-8 py-4 text-sm font-semibold text-white shadow-lg hover:bg-blue-700 transition-all hover:scale-105"
          >
            See Pricing
          </a>
        </div>
      </div>
    </section>
  )
}
