import { ImageResponse } from '@vercel/og'

export const runtime = 'edge'

export async function GET() {
  return new ImageResponse(
    (
      <div
        style={{
          height: '100%',
          width: '100%',
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'center',
          justifyContent: 'center',
          background: 'linear-gradient(135deg, #0ea5e9 0%, #6366f1 50%, #d946ef 100%)',
          fontFamily: 'system-ui, sans-serif',
        }}
      >
        {/* Decorative elements */}
        <div
          style={{
            position: 'absolute',
            top: '40px',
            right: '40px',
            width: '200px',
            height: '200px',
            borderRadius: '50%',
            background: 'rgba(255, 255, 255, 0.1)',
            filter: 'blur(60px)',
          }}
        />
        <div
          style={{
            position: 'absolute',
            bottom: '40px',
            left: '40px',
            width: '250px',
            height: '250px',
            borderRadius: '50%',
            background: 'rgba(255, 255, 255, 0.1)',
            filter: 'blur(80px)',
          }}
        />

        {/* Logo/Brand */}
        <div
          style={{
            position: 'absolute',
            top: '50px',
            left: '60px',
            fontSize: '32px',
            fontWeight: '700',
            color: 'white',
            display: 'flex',
          }}
        >
          English Liminal
        </div>

        {/* Main content */}
        <div
          style={{
            display: 'flex',
            flexDirection: 'column',
            alignItems: 'center',
            justifyContent: 'center',
            textAlign: 'center',
            padding: '0 80px',
            maxWidth: '1000px',
          }}
        >
          <h1
            style={{
              fontSize: '72px',
              fontWeight: '900',
              color: 'white',
              lineHeight: '1.1',
              margin: '0 0 30px 0',
              textShadow: '0 4px 20px rgba(0, 0, 0, 0.2)',
            }}
          >
            Stop Studying English.
            <br />
            Start Living It.
          </h1>
          <p
            style={{
              fontSize: '36px',
              fontWeight: '400',
              color: 'rgba(255, 255, 255, 0.95)',
              lineHeight: '1.4',
              margin: '0',
              textShadow: '0 2px 10px rgba(0, 0, 0, 0.15)',
            }}
          >
            Master 26 real-world scenarios across 7 professional roles
          </p>
        </div>

        {/* Trust indicators */}
        <div
          style={{
            position: 'absolute',
            bottom: '50px',
            right: '60px',
            display: 'flex',
            flexDirection: 'column',
            gap: '10px',
            alignItems: 'flex-end',
            color: 'rgba(255, 255, 255, 0.9)',
            fontSize: '22px',
          }}
        >
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
            ⭐ 4.8 Rating
          </div>
          <div style={{ display: 'flex' }}>
            10,000+ learners
          </div>
        </div>
      </div>
    ),
    {
      width: 1200,
      height: 630,
    },
  )
}
