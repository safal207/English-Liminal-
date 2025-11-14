'use client'

import { useState } from 'react'
import { ChevronDown } from 'lucide-react'

export default function FAQ() {
  const [openIndex, setOpenIndex] = useState<number | null>(0)

  const faqs = [
    {
      question: 'How is this different from Duolingo or other apps?',
      answer: 'Unlike generic apps, English Liminal focuses on real-world professional scenarios you\'ll actually use. Instead of translating "The cat drinks milk," you\'ll practice pitching investors, leading standup meetings, or negotiating a raise. Plus, our AI speech recognition gives instant feedback on pronunciation and fluency.',
    },
    {
      question: 'Do I need to be advanced to start?',
      answer: 'Not at all! We have roles for beginners (QA Engineer, Visa Journey) and advanced learners (Startup Founder, Sales Professional). Each scenario adapts to your level. Start where you are, grow as you practice.',
    },
    {
      question: 'How much time do I need to spend?',
      answer: '10-15 minutes a day is enough. Each scenario takes 5-10 minutes. Our spaced repetition system tells you when to review, so you\'re not wasting time. Consistency beats cramming.',
    },
    {
      question: 'Can I use this offline?',
      answer: 'Yes! Download scenarios ahead of time and practice offline. Your progress syncs when you\'re back online. Perfect for commutes or flights.',
    },
    {
      question: 'What if I\'m shy about speaking?',
      answer: 'That\'s exactly why English Liminal exists. Practice in private, at your own pace. No judgment, no awkward tutors. The AI gives feedback, not criticism. Build confidence before real conversations.',
    },
    {
      question: 'Is there a money-back guarantee?',
      answer: 'Absolutely. Try Premium free for 7 days. If it\'s not for you, cancel anytime during the trial—zero charge. Even after, cancel anytime with no questions asked.',
    },
    {
      question: 'What platforms does it work on?',
      answer: 'iOS, Android, and web. One subscription works everywhere. Start on your phone, continue on your laptop. Progress syncs across all devices.',
    },
    {
      question: 'Will you add more scenarios?',
      answer: 'Yes! Premium members get new scenarios every month. We\'re building Interview Prep, Conference Speaking, Networking Events, and more based on community requests. Vote on what we build next.',
    },
  ]

  return (
    <section id="faq" className="bg-white py-24 sm:py-32">
      <div className="mx-auto max-w-7xl px-6 lg:px-8">
        <div className="mx-auto max-w-2xl text-center">
          <h2 className="text-base font-semibold leading-7 text-blue-600">FAQ</h2>
          <p className="mt-2 text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
            Questions? We've Got Answers.
          </p>
          <p className="mt-6 text-lg leading-8 text-gray-600">
            Everything you need to know about English Liminal.
          </p>
        </div>

        <div className="mx-auto mt-16 max-w-3xl">
          <div className="space-y-4">
            {faqs.map((faq, index) => (
              <div
                key={index}
                className="overflow-hidden rounded-xl border border-gray-200 bg-white shadow-sm"
              >
                <button
                  onClick={() => setOpenIndex(openIndex === index ? null : index)}
                  className="flex w-full items-center justify-between p-6 text-left transition-colors hover:bg-gray-50"
                >
                  <span className="text-lg font-semibold text-gray-900">{faq.question}</span>
                  <ChevronDown
                    className={`h-5 w-5 flex-shrink-0 text-blue-600 transition-transform ${
                      openIndex === index ? 'rotate-180' : ''
                    }`}
                  />
                </button>
                <div
                  className={`overflow-hidden transition-all ${
                    openIndex === index ? 'max-h-96' : 'max-h-0'
                  }`}
                >
                  <div className="px-6 pb-6 text-gray-600 leading-relaxed">{faq.answer}</div>
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Still have questions CTA */}
        <div className="mt-16 text-center">
          <p className="text-lg text-gray-600 mb-4">Still have questions?</p>
          <a
            href="mailto:support@englishliminal.com"
            className="inline-flex items-center gap-2 text-blue-600 font-semibold hover:text-blue-700 transition-colors"
          >
            Email us at support@englishliminal.com
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M14 5l7 7m0 0l-7 7m7-7H3" />
            </svg>
          </a>
        </div>
      </div>
    </section>
  )
}
