import Link from 'next/link'

export default function Home() {
  return (
    <div>
      <div>
        <Link href="chat">
          Chat
        </Link>
      </div>
      <div>
        <Link href="about">
          About
        </Link>
      </div>
      <div>
        <Link href="members">
          Members
        </Link>
      </div>
    </div>
  )
}
