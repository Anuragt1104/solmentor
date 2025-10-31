'use client';

import { useWallet } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import Link from 'next/link';
import { motion } from 'framer-motion';

export default function Home() {
  const { connected } = useWallet();

  return (
    <main className="min-h-screen bg-gradient-to-br from-gray-900 via-purple-900 to-gray-900">
      {/* Navigation */}
      <nav className="container mx-auto px-6 py-6 flex justify-between items-center">
        <div className="text-2xl font-bold gradient-text">
          🎓 SolMentor
        </div>
        <WalletMultiButton />
      </nav>

      {/* Hero Section */}
      <section className="container mx-auto px-6 py-20 text-center">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.8 }}
        >
          <h1 className="text-6xl font-bold mb-6">
            Learn Solana<br />
            <span className="gradient-text">Earn Rewards</span>
          </h1>
          <p className="text-xl text-gray-300 mb-8 max-w-2xl mx-auto">
            Master blockchain development with AI-powered quizzes, earn on-chain achievements,
            and compete with developers worldwide. Built on Solana with AImpact.dev
          </p>

          {!connected ? (
            <div className="space-y-4">
              <p className="text-lg text-purple-400">
                👆 Connect your wallet to get started
              </p>
            </div>
          ) : (
            <div className="flex gap-4 justify-center">
              <Link
                href="/learn"
                className="px-8 py-4 bg-purple-600 hover:bg-purple-700 rounded-lg font-semibold transition-all hover:scale-105"
              >
                Start Learning
              </Link>
              <Link
                href="/profile"
                className="px-8 py-4 bg-gray-700 hover:bg-gray-600 rounded-lg font-semibold transition-all hover:scale-105"
              >
                My Profile
              </Link>
            </div>
          )}
        </motion.div>
      </section>

      {/* Features Grid */}
      <section className="container mx-auto px-6 py-20">
        <div className="grid md:grid-cols-3 gap-8">
          {features.map((feature, index) => (
            <motion.div
              key={index}
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.5, delay: index * 0.1 }}
              className="p-6 bg-gray-800 rounded-xl card-hover"
            >
              <div className="text-4xl mb-4">{feature.icon}</div>
              <h3 className="text-xl font-bold mb-2">{feature.title}</h3>
              <p className="text-gray-400">{feature.description}</p>
            </motion.div>
          ))}
        </div>
      </section>

      {/* Stats Section */}
      <section className="container mx-auto px-6 py-20">
        <div className="grid md:grid-cols-4 gap-8 text-center">
          {stats.map((stat, index) => (
            <motion.div
              key={index}
              initial={{ opacity: 0, scale: 0.9 }}
              animate={{ opacity: 1, scale: 1 }}
              transition={{ duration: 0.5, delay: index * 0.1 }}
              className="p-6 bg-gray-800 rounded-xl"
            >
              <div className="text-4xl font-bold gradient-text mb-2">
                {stat.value}
              </div>
              <div className="text-gray-400">{stat.label}</div>
            </motion.div>
          ))}
        </div>
      </section>

      {/* CTA Section */}
      <section className="container mx-auto px-6 py-20 text-center">
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ duration: 0.8, delay: 0.5 }}
          className="bg-gradient-to-r from-purple-600 to-blue-600 rounded-2xl p-12"
        >
          <h2 className="text-4xl font-bold mb-4">
            Ready to Start Your Journey?
          </h2>
          <p className="text-xl mb-8 opacity-90">
            Join thousands of developers learning and earning on Solana
          </p>
          {!connected ? (
            <WalletMultiButton className="!bg-white !text-purple-600 hover:!bg-gray-100" />
          ) : (
            <Link
              href="/learn"
              className="inline-block px-8 py-4 bg-white text-purple-600 rounded-lg font-semibold hover:bg-gray-100 transition-all hover:scale-105"
            >
              Start Learning Now →
            </Link>
          )}
        </motion.div>
      </section>

      {/* Footer */}
      <footer className="container mx-auto px-6 py-8 text-center text-gray-400 border-t border-gray-800">
        <p>
          Built with ❤️ using{' '}
          <a href="https://aimpact.dev" target="_blank" rel="noopener noreferrer" className="text-purple-400 hover:text-purple-300">
            AImpact.dev
          </a>
          {' '}for the AImpact Sidetrack Hackathon
        </p>
        <div className="mt-4 space-x-6">
          <a href="https://twitter.com/solmentor" className="hover:text-purple-400">Twitter</a>
          <a href="https://discord.gg/solmentor" className="hover:text-purple-400">Discord</a>
          <a href="https://github.com/yourusername/solmentor" className="hover:text-purple-400">GitHub</a>
        </div>
      </footer>
    </main>
  );
}

const features = [
  {
    icon: '🤖',
    title: 'AI-Powered Quizzes',
    description: 'Dynamic, personalized learning challenges that adapt to your skill level',
  },
  {
    icon: '🏆',
    title: 'On-Chain Achievements',
    description: 'Earn permanent NFT badges stored on the Solana blockchain',
  },
  {
    icon: '💰',
    title: 'Token Rewards',
    description: 'Get paid in $MENTOR tokens for completing learning challenges',
  },
  {
    icon: '📊',
    title: 'Progress Tracking',
    description: 'Your learning journey stored permanently on-chain',
  },
  {
    icon: '🎮',
    title: 'Gamification',
    description: 'XP points, levels, streaks, and competitive leaderboards',
  },
  {
    icon: '👥',
    title: 'Creator Economy',
    description: 'Build and monetize your own learning content',
  },
];

const stats = [
  { value: '10K+', label: 'Learners' },
  { value: '50K+', label: 'Quizzes Completed' },
  { value: '5K+', label: 'NFTs Minted' },
  { value: '$100K+', label: 'Rewards Earned' },
];
