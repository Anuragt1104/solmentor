import './globals.css';
import type { Metadata } from 'next';
import { Inter } from 'next/font/google';
import { WalletProvider } from '@/components/WalletProvider';
import { Toaster } from 'react-hot-toast';

const inter = Inter({ subsets: ['latin'] });

export const metadata: Metadata = {
  title: 'SolMentor - Learn Solana & Earn Rewards',
  description: 'AI-Powered Decentralized Learning Platform on Solana',
  keywords: ['solana', 'blockchain', 'education', 'web3', 'learn-to-earn'],
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <WalletProvider>
          {children}
          <Toaster position="bottom-right" />
        </WalletProvider>
      </body>
    </html>
  );
}
