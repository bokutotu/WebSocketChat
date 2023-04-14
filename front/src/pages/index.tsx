import Image from 'next/image'
import { Inter } from 'next/font/google'
import Link from 'next/link'

const inter = Inter({ subsets: ['latin'] })

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
    </div>
  )
}
