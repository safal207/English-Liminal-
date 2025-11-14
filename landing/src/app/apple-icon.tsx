import { ImageResponse } from 'next/og'

export const runtime = 'edge'
export const size = {
  width: 180,
  height: 180,
}
export const contentType = 'image/png'

export default function AppleIcon() {
  return new ImageResponse(
    (
      <div
        style={{
          width: '100%',
          height: '100%',
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'center',
          justifyContent: 'center',
          background: 'linear-gradient(135deg, #0ea5e9 0%, #6366f1 50%, #d946ef 100%)',
          borderRadius: '40px',
        }}
      >
        <div
          style={{
            fontSize: '90px',
            fontWeight: '900',
            color: 'white',
            fontFamily: 'sans-serif',
            marginBottom: '10px',
          }}
        >
          L
        </div>
        <div
          style={{
            fontSize: '20px',
            fontWeight: '600',
            color: 'rgba(255, 255, 255, 0.95)',
            fontFamily: 'sans-serif',
          }}
        >
          Liminal
        </div>
      </div>
    ),
    {
      ...size,
    }
  )
}
