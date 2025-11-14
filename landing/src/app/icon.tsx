import { ImageResponse } from 'next/og'

export const runtime = 'edge'
export const size = {
  width: 32,
  height: 32,
}
export const contentType = 'image/png'

export default function Icon() {
  return new ImageResponse(
    (
      <div
        style={{
          width: '100%',
          height: '100%',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          background: 'linear-gradient(135deg, #0ea5e9 0%, #d946ef 100%)',
          borderRadius: '4px',
        }}
      >
        <div
          style={{
            fontSize: '20px',
            fontWeight: '900',
            color: 'white',
            fontFamily: 'sans-serif',
          }}
        >
          L
        </div>
      </div>
    ),
    {
      ...size,
    }
  )
}
