import Link from 'next/link'

export default function Home() {
  return (
    <div>
      <div>
        <Link href="/about">
          about
        </Link>
      </div>
      <div>
        <Link href="/chat">
          chat
        </Link>
      </div>
    </div>
  )
}
