# SolMentor Frontend Architecture

This document explains the Next.js frontend of SolMentor, covering the application structure, Solana wallet integration, UI/UX design decisions, and how the frontend communicates with the on-chain program.

## Table of Contents

1. [Technology Overview](#technology-overview)
2. [Project Structure](#project-structure)
3. [Wallet Integration](#wallet-integration)
4. [Application Layout](#application-layout)
5. [Landing Page Breakdown](#landing-page-breakdown)
6. [Styling System](#styling-system)
7. [Smart Contract Integration](#smart-contract-integration)
8. [User Experience Flow](#user-experience-flow)

## Technology Overview

### Core Technologies

**Next.js 14**
- React-based framework with server-side rendering capabilities
- App Router architecture for modern routing
- Built-in optimization for images, fonts, and scripts
- TypeScript support for type safety

**Solana Wallet Adapter**
- Official library for Solana wallet integration
- Supports multiple wallet providers (Phantom, Solflare, Backpack)
- React hooks for easy wallet state management
- Auto-connect functionality for seamless UX

**Styling Stack**
- Tailwind CSS: Utility-first CSS framework
- Custom CSS variables for theming
- Framer Motion: Animation library for smooth transitions
- React Hot Toast: Beautiful notification system

**Additional Libraries**
- @coral-xyz/anchor: Client-side Anchor integration
- @solana/web3.js: Core Solana functionality
- Zustand: Planned for state management (dependency installed)

## Project Structure

```
frontend/
├── app/                          # Next.js App Router
│   ├── layout.tsx               # Root layout with providers
│   ├── page.tsx                 # Landing page
│   └── globals.css              # Global styles
│
├── components/                   # React components
│   └── WalletProvider.tsx       # Wallet adapter wrapper
│
├── lib/                         # Utility functions (planned)
│   ├── anchor/                  # Anchor program client
│   ├── utils/                   # Helper functions
│   └── constants.ts             # App constants
│
├── public/                      # Static assets
│   └── (images, icons, etc.)
│
├── next.config.mjs              # Next.js configuration
├── tailwind.config.ts           # Tailwind configuration
└── tsconfig.json                # TypeScript configuration
```

## Wallet Integration

The wallet integration is the bridge between users and the Solana blockchain. Let's break down how it works.

### WalletProvider Component

**Location**: `components/WalletProvider.tsx`

**Purpose**: Wraps the entire application to provide wallet functionality to all components.

**Code Explanation**:

```typescript
'use client';

import { FC, ReactNode, useMemo } from 'react';
import { ConnectionProvider, WalletProvider as SolanaWalletProvider } from '@solana/wallet-adapter-react';
import { WalletAdapterNetwork } from '@solana/wallet-adapter-base';
import { WalletModalProvider } from '@solana/wallet-adapter-react-ui';
import {
  PhantomWalletAdapter,
  SolflareWalletAdapter,
  BackpackWalletAdapter,
} from '@solana/wallet-adapter-wallets';
import { clusterApiUrl } from '@solana/web3.js';

import '@solana/wallet-adapter-react-ui/styles.css';

export const WalletProvider: FC<{ children: ReactNode }> = ({ children }) => {
  // Network selection: defaults to Devnet
  const network = (process.env.NEXT_PUBLIC_SOLANA_NETWORK as WalletAdapterNetwork) || WalletAdapterNetwork.Devnet;
  
  // RPC endpoint: custom or default Devnet RPC
  const endpoint = useMemo(
    () => process.env.NEXT_PUBLIC_RPC_ENDPOINT || clusterApiUrl(network),
    [network]
  );

  // Supported wallets
  const wallets = useMemo(
    () => [
      new PhantomWalletAdapter(),
      new SolflareWalletAdapter(),
      new BackpackWalletAdapter(),
    ],
    []
  );

  return (
    <ConnectionProvider endpoint={endpoint}>
      <SolanaWalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>
          {children}
        </WalletModalProvider>
      </SolanaWalletProvider>
    </ConnectionProvider>
  );
};
```

### Key Concepts Explained

**1. Client-Side Rendering**

```typescript
'use client';
```

This directive tells Next.js to render this component on the client side only. Wallet adapters require browser APIs (like window.solana) that aren't available during server-side rendering.

**2. Network Configuration**

```typescript
const network = (process.env.NEXT_PUBLIC_SOLANA_NETWORK as WalletAdapterNetwork) || WalletAdapterNetwork.Devnet;
```

The network can be configured via environment variables:
- Development: Devnet (test SOL, free)
- Production: Mainnet-beta (real SOL)
- Local: Localhost (for Anchor local testing)

**Why configurable?** Different environments need different networks without code changes.

**3. RPC Endpoint**

```typescript
const endpoint = useMemo(
  () => process.env.NEXT_PUBLIC_RPC_ENDPOINT || clusterApiUrl(network),
  [network]
);
```

The RPC endpoint is the server the app talks to for blockchain data:
- Default: Solana's public RPC (rate-limited but free)
- Custom: Your own RPC (HeliusHelius, QuickNode, etc.) for better performance

**useMemo optimization**: The endpoint calculation only runs when network changes, preventing unnecessary recalculations.

**4. Wallet Adapters**

```typescript
const wallets = useMemo(
  () => [
    new PhantomWalletAdapter(),
    new SolflareWalletAdapter(),
    new BackpackWalletAdapter(),
  ],
  []
);
```

We support three popular Solana wallets:
- **Phantom**: Most popular, beginner-friendly
- **Solflare**: Feature-rich, security-focused
- **Backpack**: Modern, xNFT-enabled

**Empty dependency array**: Wallet instances are created once and reused.

**5. Provider Hierarchy**

```typescript
<ConnectionProvider endpoint={endpoint}>
  <SolanaWalletProvider wallets={wallets} autoConnect>
    <WalletModalProvider>
      {children}
    </WalletModalProvider>
  </SolanaWalletProvider>
</ConnectionProvider>
```

Three nested providers:
- **ConnectionProvider**: Manages the RPC connection
- **WalletProvider**: Manages wallet state and switching
- **WalletModalProvider**: Provides the wallet selection modal UI

The `autoConnect` prop automatically connects to the last-used wallet on page load, improving UX.

### Using Wallet in Components

Any component inside the WalletProvider can access wallet functionality:

```typescript
import { useWallet } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';

function MyComponent() {
  const { connected, publicKey, signTransaction } = useWallet();
  
  if (!connected) {
    return <WalletMultiButton />;
  }
  
  return (
    <div>
      <p>Connected: {publicKey.toBase58()}</p>
      {/* Your app logic */}
    </div>
  );
}
```

**Useful hooks and values**:
- `connected`: Boolean - is a wallet connected?
- `publicKey`: PublicKey | null - the user's wallet address
- `signTransaction`: Function - sign transactions
- `sendTransaction`: Function - sign and send transactions

## Application Layout

### Root Layout

**Location**: `app/layout.tsx`

**Purpose**: The root layout wraps all pages and provides global context.

**Code Breakdown**:

```typescript
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
```

**Key Elements**:

**1. Font Loading**

```typescript
const inter = Inter({ subsets: ['latin'] });
```

Next.js optimizes font loading:
- Loads Inter font from Google Fonts
- Only includes Latin characters (smaller file size)
- Automatically prevents layout shift

**2. Metadata**

```typescript
export const metadata: Metadata = {
  title: 'SolMentor - Learn Solana & Earn Rewards',
  description: 'AI-Powered Decentralized Learning Platform on Solana',
  keywords: ['solana', 'blockchain', 'education', 'web3', 'learn-to-earn'],
};
```

SEO optimization:
- Appears in browser tabs and bookmarks
- Used by search engines for indexing
- Displayed in social media previews

**3. Toaster Component**

```typescript
<Toaster position="bottom-right" />
```

Global notification system:
- Success messages: "Profile created!"
- Error alerts: "Transaction failed"
- Loading states: "Processing quiz..."

Positioned bottom-right to not obscure wallet button (top-right).

## Landing Page Breakdown

**Location**: `app/page.tsx`

The landing page is the first impression users have of SolMentor. Let's analyze each section.

### Navigation Bar

```typescript
<nav className="container mx-auto px-6 py-6 flex justify-between items-center">
  <div className="text-2xl font-bold gradient-text">
    🎓 SolMentor
  </div>
  <WalletMultiButton />
</nav>
```

**Design Decisions**:
- Logo on left (standard web convention)
- Wallet button on right (prominent placement)
- Gradient text effect for brand identity
- Clean, uncluttered layout

The `WalletMultiButton` is a pre-built component that:
- Shows "Select Wallet" when disconnected
- Shows wallet address (shortened) when connected
- Opens modal to select/change wallet
- Includes copy address and disconnect options

### Hero Section

```typescript
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
    {/* CTA buttons */}
  </motion.div>
</section>
```

**Framer Motion Animation**:
- `initial`: Starting state (invisible, slightly below)
- `animate`: End state (visible, correct position)
- `transition`: 0.8 second smooth transition

This creates a fade-in-up effect that draws attention without being jarring.

**Value Proposition**:
- Headline: Clear benefit ("Learn" + "Earn")
- Subheadline: Explains how it works
- CTA: Different buttons for connected vs. disconnected states

### Conditional CTAs

```typescript
{!connected ? (
  <div className="space-y-4">
    <p className="text-lg text-purple-400">
      👆 Connect your wallet to get started
    </p>
  </div>
) : (
  <div className="flex gap-4 justify-center">
    <Link href="/learn" className="...">Start Learning</Link>
    <Link href="/profile" className="...">My Profile</Link>
  </div>
)}
```

**UX Logic**:
- **Not connected**: Prompt to connect wallet
- **Connected**: Show action buttons (Learn, Profile)

This guides users through the necessary first step (wallet connection) before showing unavailable features.

### Features Grid

```typescript
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
```

**Staggered Animation**:
```typescript
delay: index * 0.1
```

Each card animates 0.1 seconds after the previous one, creating a pleasant cascade effect.

**Feature Cards**:
1. AI-Powered Quizzes
2. On-Chain Achievements
3. Token Rewards
4. Progress Tracking
5. Gamification
6. Creator Economy

These six features communicate the value proposition visually.

### Stats Section

```typescript
const stats = [
  { value: '10K+', label: 'Learners' },
  { value: '50K+', label: 'Quizzes Completed' },
  { value: '5K+', label: 'NFTs Minted' },
  { value: '$100K+', label: 'Rewards Earned' },
];
```

**Social Proof**: Large numbers build credibility and FOMO (Fear Of Missing Out).

**Why these metrics?**
- Learners: Community size
- Quizzes: Platform activity
- NFTs: On-chain proof
- Rewards: Real value

Note: These are aspirational stats for marketing. In production, these should be pulled from on-chain data or analytics.

### Footer

```typescript
<footer className="container mx-auto px-6 py-8 text-center text-gray-400 border-t border-gray-800">
  <p>
    Built with ❤️ using{' '}
    <a href="https://aimpact.dev" target="_blank" rel="noopener noreferrer">
      AImpact.dev
    </a>
    {' '}for the AImpact Sidetrack Hackathon
  </p>
  <div className="mt-4 space-x-6">
    <a href="https://twitter.com/solmentor">Twitter</a>
    <a href="https://discord.gg/solmentor">Discord</a>
    <a href="https://github.com/yourusername/solmentor">GitHub</a>
  </div>
</footer>
```

**Hackathon Context**: Mentions AImpact.dev to satisfy submission requirements.

**Social Links**: Build community and allow users to follow updates.

## Styling System

### Tailwind CSS

Tailwind is a utility-first CSS framework. Instead of writing custom CSS classes, you compose styles from utility classes.

**Example**:
```html
<!-- Traditional CSS -->
<div class="feature-card">...</div>
<style>.feature-card { padding: 1.5rem; background: #1f2937; border-radius: 0.75rem; }</style>

<!-- Tailwind CSS -->
<div class="p-6 bg-gray-800 rounded-xl">...</div>
```

Benefits:
- No context switching (HTML and styles in one place)
- Smaller CSS bundle (unused styles are purged)
- Consistent design system (predefined values)

### Custom CSS Variables

**Location**: `app/globals.css`

```css
:root {
  --background: #0d0d0d;
  --foreground: #ffffff;
}
```

These variables define the base color scheme:
- Dark background (#0d0d0d - almost black)
- White text (#ffffff)

Can be changed for theme switching (light mode, custom themes).

### Gradient Text Effect

```css
.gradient-text {
  background: linear-gradient(90deg, #9945FF 0%, #14F195 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}
```

**How it works**:
1. Create a gradient background (purple to green)
2. Clip the background to the text shape
3. Make the text fill transparent
4. Result: Text filled with gradient

**Brand colors**:
- #9945FF: Solana purple
- #14F195: Solana green

This ties the visual identity to the Solana ecosystem.

### Card Hover Effect

```css
.card-hover {
  transition: all 0.3s ease;
}

.card-hover:hover {
  transform: translateY(-4px);
  box-shadow: 0 20px 40px rgba(153, 69, 255, 0.3);
}
```

**Effect**: Cards lift up slightly and glow purple on hover.

**Why?**
- Provides feedback (card is interactive)
- Adds polish and premium feel
- Draws attention to content

### Wallet Button Styling

```css
.wallet-adapter-button {
  background-color: #9945FF !important;
}

.wallet-adapter-button:not([disabled]):hover {
  background-color: #7a36cc !important;
}
```

Overrides default wallet adapter styles to match Solana brand colors.

**!important**: Necessary to override library defaults.

### Custom Scrollbar

```css
::-webkit-scrollbar {
  width: 8px;
}

::-webkit-scrollbar-thumb {
  background: #9945FF;
  border-radius: 4px;
}
```

Even the scrollbar matches the brand (purple) for a cohesive design.

## Smart Contract Integration

While the current MVP focuses on the frontend UI, here's how the Anchor program integration works.

### Anchor Client Setup (Planned)

```typescript
// lib/anchor/client.ts (example structure)
import { Program, AnchorProvider, web3 } from '@coral-xyz/anchor';
import { useAnchorWallet, useConnection } from '@solana/wallet-adapter-react';
import { IDL } from './idl/solmentor';

export function useAnchorProgram() {
  const wallet = useAnchorWallet();
  const { connection } = useConnection();
  
  if (!wallet) return null;
  
  const provider = new AnchorProvider(connection, wallet, {});
  const programId = new web3.PublicKey('SoLMnToR11111111111111111111111111111111111');
  const program = new Program(IDL, programId, provider);
  
  return program;
}
```

### Creating a Profile

```typescript
// Example usage in a component
async function createProfile(username: string) {
  const program = useAnchorProgram();
  if (!program || !wallet.publicKey) return;
  
  const [userProfilePDA] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from('user_profile'), wallet.publicKey.toBuffer()],
    program.programId
  );
  
  try {
    const tx = await program.methods
      .initializeProfile(username)
      .accounts({
        userProfile: userProfilePDA,
        authority: wallet.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();
      
    toast.success('Profile created!');
    console.log('Transaction:', tx);
  } catch (error) {
    toast.error('Failed to create profile');
    console.error(error);
  }
}
```

### Submitting a Quiz

```typescript
async function submitQuiz(quizId: string, score: number, total: number) {
  const program = useAnchorProgram();
  if (!program || !wallet.publicKey) return;
  
  // Derive PDAs
  const [userProfilePDA] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from('user_profile'), wallet.publicKey.toBuffer()],
    program.programId
  );
  
  const [quizResultPDA] = web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from('quiz_result'),
      wallet.publicKey.toBuffer(),
      Buffer.from(quizId)
    ],
    program.programId
  );
  
  try {
    const tx = await program.methods
      .submitQuiz(quizId, score, total)
      .accounts({
        userProfile: userProfilePDA,
        quizResult: quizResultPDA,
        authority: wallet.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();
      
    toast.success(`Quiz complete! Earned ${score * 10} XP`);
  } catch (error) {
    toast.error('Failed to submit quiz');
    console.error(error);
  }
}
```

### Fetching Profile Data

```typescript
async function fetchUserProfile() {
  const program = useAnchorProgram();
  if (!program || !wallet.publicKey) return null;
  
  const [userProfilePDA] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from('user_profile'), wallet.publicKey.toBuffer()],
    program.programId
  );
  
  try {
    const profile = await program.account.userProfile.fetch(userProfilePDA);
    return {
      username: profile.username,
      xp: profile.xp.toNumber(),
      level: profile.level.toNumber(),
      streak: profile.streak.toNumber(),
      quizzesCompleted: profile.quizzesCompleted.toNumber(),
      achievementsEarned: profile.achievementsEarned.toNumber(),
    };
  } catch (error) {
    // Profile doesn't exist yet
    return null;
  }
}
```

## User Experience Flow

### First-Time User Journey

1. **Landing Page**
   - User sees hero section
   - Learns about SolMentor features
   - Motivated by stats and social proof

2. **Wallet Connection**
   - Clicks "Select Wallet" button
   - Modal opens with wallet options
   - Selects Phantom (or other)
   - Browser extension prompts for approval
   - Connection successful

3. **Profile Creation**
   - "Start Learning" button now visible
   - Clicks to /learn page
   - Prompted to create profile
   - Enters username
   - Signs transaction (pays small rent)
   - Profile created on-chain

4. **First Quiz**
   - Sees list of available quizzes
   - Selects "Solana Basics"
   - Answers 10 multiple-choice questions
   - Submits quiz
   - Signs transaction
   - Sees results: "8/10 - You earned 80 XP!"
   - Profile updates: Level 1 → 1, XP: 80

5. **Ongoing Engagement**
   - Returns daily for streak
   - Takes more quizzes, earns XP
   - Reaches milestones, unlocks achievements
   - Checks leaderboard
   - Shares progress on social media

### Error Handling

**No Wallet Extension**
```typescript
if (!wallet) {
  return (
    <div className="alert">
      <p>No wallet detected. Please install Phantom wallet.</p>
      <a href="https://phantom.app/">Download Phantom</a>
    </div>
  );
}
```

**Transaction Failure**
```typescript
try {
  await program.methods.submitQuiz(...).rpc();
  toast.success('Quiz submitted!');
} catch (error) {
  if (error.message.includes('User rejected')) {
    toast.error('Transaction cancelled');
  } else if (error.message.includes('insufficient funds')) {
    toast.error('Not enough SOL for transaction fee');
  } else {
    toast.error('Transaction failed. Please try again.');
  }
}
```

**Network Issues**
```typescript
const { connection } = useConnection();
const [isHealthy, setIsHealthy] = useState(true);

useEffect(() => {
  const checkHealth = async () => {
    try {
      await connection.getLatestBlockhash();
      setIsHealthy(true);
    } catch {
      setIsHealthy(false);
      toast.error('Network issues. Please refresh.');
    }
  };
  
  checkHealth();
  const interval = setInterval(checkHealth, 30000);
  return () => clearInterval(interval);
}, [connection]);
```

## Performance Optimizations

### Image Optimization

```typescript
import Image from 'next/image';

<Image 
  src="/logo.png" 
  alt="SolMentor" 
  width={200} 
  height={200}
  priority // Load immediately for above-fold content
/>
```

Next.js automatically:
- Resizes images for device size
- Serves modern formats (WebP)
- Lazy loads images below fold
- Prevents layout shift

### Code Splitting

Next.js automatically splits code by route:
- `/` loads only landing page code
- `/learn` loads only learning page code
- Shared code is in common bundle

This keeps initial load fast.

### Memo and Callbacks

```typescript
const wallets = useMemo(() => [
  new PhantomWalletAdapter(),
  // ...
], []); // Only create wallet instances once

const handleSubmit = useCallback(async (quiz) => {
  // ...
}, [program, wallet]); // Only recreate if dependencies change
```

Prevents unnecessary re-renders and re-instantiations.

## Future Frontend Enhancements

### State Management
- Implement Zustand for global state (user profile, quiz data)
- Cache on-chain data to reduce RPC calls
- Optimistic updates for better UX

### Additional Pages
- `/learn`: Quiz selection and taking interface
- `/profile`: User stats, achievements, history
- `/leaderboard`: Top learners rankings
- `/create`: Content creator dashboard

### Real-Time Features
- WebSocket connection for live leaderboard updates
- Push notifications for streak reminders
- Real-time XP animations

### Progressive Web App
- Add service worker for offline functionality
- Install prompt for mobile users
- Push notifications for engagement

### Analytics
- Track user interactions (quiz completions, time spent)
- A/B test CTAs and messaging
- Conversion funnel analysis

## Conclusion

The SolMentor frontend is built with modern web technologies and Solana-specific tools to create a seamless learning experience. The architecture prioritizes:

- **User Experience**: Smooth animations, clear CTAs, intuitive flow
- **Performance**: Code splitting, optimized assets, efficient rendering
- **Web3 Integration**: Solana wallet adapter, Anchor client setup
- **Scalability**: Component-based architecture, modular design
- **Maintainability**: TypeScript, clear structure, documented code

As the platform grows, the frontend is architected to easily add new features, integrate more sophisticated smart contract interactions, and scale to thousands of concurrent learners.

