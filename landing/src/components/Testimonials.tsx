import { Star, Quote } from 'lucide-react'

export default function Testimonials() {
  const testimonials = [
    {
      name: 'Alex Martinez',
      role: 'Startup Founder',
      location: 'Berlin, Germany',
      avatar: 'AM',
      rating: 5,
      text: '"I went from nervous stumbling to confidently pitching VCs in 3 weeks. The investor pitch scenario is gold. Raised €500K two days after my last practice session."',
      highlight: 'Raised €500K after 3 weeks',
    },
    {
      name: 'Maria Santos',
      role: 'Senior Developer',
      location: 'Lisbon, Portugal',
      avatar: 'MS',
      rating: 5,
      text: '"Code reviews were terrifying in English. Now I\'m leading technical discussions and got promoted to Tech Lead. The remote developer scenarios nailed exactly what I needed."',
      highlight: 'Promoted to Tech Lead',
    },
    {
      name: 'Raj Patel',
      role: 'Software Engineer',
      location: 'Dubai, UAE',
      avatar: 'RP',
      rating: 5,
      text: '"Visa interview was in 5 days. I practiced the scenarios every morning. Walked in confident, walked out approved. Best $10 I ever spent."',
      highlight: 'Visa approved in 5 days',
    },
  ]

  const stats = [
    { label: 'Active Learners', value: '10,000+' },
    { label: 'Scenarios Completed', value: '250,000+' },
    { label: 'Average Rating', value: '4.8/5' },
    { label: 'Countries', value: '120+' },
  ]

  return (
    <section className="bg-gradient-to-b from-white to-gray-50 py-24 sm:py-32">
      <div className="mx-auto max-w-7xl px-6 lg:px-8">
        {/* Stats */}
        <div className="mx-auto max-w-2xl lg:max-w-none">
          <div className="grid grid-cols-2 gap-8 lg:grid-cols-4">
            {stats.map((stat) => (
              <div key={stat.label} className="text-center">
                <div className="text-4xl font-bold text-blue-600">{stat.value}</div>
                <div className="mt-2 text-sm text-gray-600">{stat.label}</div>
              </div>
            ))}
          </div>
        </div>

        {/* Testimonials Header */}
        <div className="mx-auto max-w-2xl text-center mt-24">
          <h2 className="text-base font-semibold leading-7 text-blue-600">Success Stories</h2>
          <p className="mt-2 text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
            Real People. Real Results.
          </p>
          <p className="mt-6 text-lg leading-8 text-gray-600">
            See what learners accomplished after practicing with English Liminal.
          </p>
        </div>

        {/* Testimonials Grid */}
        <div className="mx-auto mt-16 grid max-w-2xl grid-cols-1 gap-8 lg:max-w-none lg:grid-cols-3">
          {testimonials.map((testimonial) => (
            <div
              key={testimonial.name}
              className="relative bg-white rounded-2xl p-8 shadow-lg ring-1 ring-gray-200 hover:shadow-xl transition-shadow"
            >
              {/* Quote icon */}
              <Quote className="absolute top-8 right-8 h-8 w-8 text-blue-100" />

              {/* Rating */}
              <div className="flex items-center gap-1 mb-4">
                {[...Array(testimonial.rating)].map((_, i) => (
                  <Star key={i} className="w-4 h-4 fill-yellow-400 text-yellow-400" />
                ))}
              </div>

              {/* Testimonial text */}
              <p className="text-gray-700 leading-relaxed relative z-10">
                {testimonial.text}
              </p>

              {/* Highlight badge */}
              <div className="mt-4 inline-flex items-center rounded-full bg-green-50 px-3 py-1 text-sm font-semibold text-green-700">
                {testimonial.highlight}
              </div>

              {/* Author */}
              <div className="mt-6 flex items-center gap-3">
                <div className="flex h-12 w-12 items-center justify-center rounded-full bg-gradient-to-br from-blue-600 to-purple-600 text-white font-semibold">
                  {testimonial.avatar}
                </div>
                <div>
                  <div className="font-semibold text-gray-900">{testimonial.name}</div>
                  <div className="text-sm text-gray-600">
                    {testimonial.role} • {testimonial.location}
                  </div>
                </div>
              </div>
            </div>
          ))}
        </div>

        {/* CTA */}
        <div className="mt-16 text-center">
          <p className="text-lg text-gray-600 mb-6">
            Join 10,000+ professionals improving their English every day.
          </p>
          <a
            href="#pricing"
            className="inline-flex items-center gap-2 rounded-full bg-blue-600 px-8 py-4 text-sm font-semibold text-white shadow-lg hover:bg-blue-700 transition-all hover:scale-105"
          >
            Start Your Journey
          </a>
        </div>
      </div>
    </section>
  )
}
