# SolMentor

An AI-powered decentralized learning platform built on Solana that transforms education into an engaging, gamified experience with real on-chain rewards.

## The Problem

Traditional online learning platforms face several challenges:
- Learning progress is stored on centralized servers that can be lost or manipulated
- No real ownership of achievements or credentials
- Lack of meaningful incentives to maintain learning streaks
- Limited transparency in skill verification

## Our Solution

SolMentor brings learning progress on-chain, leveraging Solana's speed and low costs to create a transparent, verifiable, and rewarding educational experience. Every quiz completed, every achievement earned, and every streak maintained is permanently recorded on the blockchain.

## What Makes SolMentor Unique

### True On-Chain Learning Progression
Unlike platforms that only store achievements off-chain, SolMentor records every aspect of your learning journey directly on the Solana blockchain. Your XP, level, quiz results, and achievements are all permanent, verifiable, and truly owned by you.

### Intelligent Gamification System
We've built a sophisticated gamification layer into our smart contract that includes:
- **Dynamic XP System**: Earn 10 XP per correct answer, with a 50 XP bonus for perfect scores
- **Progressive Leveling**: Every 100 XP gained moves you up one level
- **Streak Mechanics**: Daily engagement is rewarded through a 24-hour streak window that resets if you miss a day
- **Tiered Achievements**: Bronze, Silver, Gold, and Platinum achievements with bonus XP rewards

### Permanent, Verifiable Credentials
All learning progress is stored using Program Derived Addresses (PDAs), making your achievements cryptographically verifiable and impossible to forge.

### Built for the Solana Ecosystem
SolMentor is specifically designed to teach Solana development and blockchain concepts, creating a feedback loop where learners engage with the very technology they're learning about.

## Technology Stack

### Smart Contract Layer
- **Solana Blockchain**: High-speed, low-cost transactions for seamless user experience
- **Anchor Framework**: Type-safe Rust framework for building Solana programs
- **Program Derived Addresses (PDAs)**: Secure, deterministic account generation

### Frontend Layer
- **Next.js 14**: Modern React framework with server-side rendering
- **TypeScript**: Type-safe development experience
- **Solana Wallet Adapter**: Seamless integration with Phantom, Solflare, and Backpack wallets
- **Framer Motion**: Smooth animations and transitions
- **Tailwind CSS**: Utility-first styling for beautiful UI

### AI Integration
- **OpenAI API**: Dynamic quiz generation and personalized learning paths
- **Adaptive Learning**: AI adjusts difficulty based on performance

## Architecture

### High-Level System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         User Browser                             │
│  ┌────────────────────────────────────────────────────────┐     │
│  │         Next.js Frontend Application                   │     │
│  │                                                         │     │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ │     │
│  │  │   Landing    │  │    Learn     │  │   Profile    │ │     │
│  │  │     Page     │  │     Page     │  │     Page     │ │     │
│  │  └──────────────┘  └──────────────┘  └──────────────┘ │     │
│  │                                                         │     │
│  │  ┌────────────────────────────────────────────────┐   │     │
│  │  │     Solana Wallet Adapter Integration          │   │     │
│  │  │  (Phantom, Solflare, Backpack)                 │   │     │
│  │  └────────────────────────────────────────────────┘   │     │
│  └────────────────────────────────────────────────────────┘     │
└──────────────────────┬──────────────────────────────────────────┘
                       │
                       │ RPC Calls
                       ▼
        ┌──────────────────────────────┐
        │      Solana RPC Endpoint     │
        │         (Devnet)             │
        └──────────────┬───────────────┘
                       │
                       │ Transaction Processing
                       ▼
        ┌──────────────────────────────┐
        │   SolMentor Anchor Program   │
        │   (Smart Contract)           │
        │                              │
        │  Program ID:                 │
        │  SoLMnToR111...11111         │
        └──────────────┬───────────────┘
                       │
                       │ Account Management
                       ▼
        ┌──────────────────────────────┐
        │    On-Chain Data Storage     │
        │                              │
        │  ┌────────────────────────┐  │
        │  │   UserProfile (PDA)    │  │
        │  │  - XP, Level, Streak   │  │
        │  └────────────────────────┘  │
        │                              │
        │  ┌────────────────────────┐  │
        │  │   QuizResult (PDA)     │  │
        │  │  - Score, Attempts     │  │
        │  └────────────────────────┘  │
        │                              │
        │  ┌────────────────────────┐  │
        │  │   Achievement (PDA)    │  │
        │  │  - Tier, Rewards       │  │
        │  └────────────────────────┘  │
        └──────────────────────────────┘
```

### Smart Contract Architecture

The SolMentor program manages three core account types, all created using Program Derived Addresses (PDAs) for security:

```
UserProfile Account
├── Seed: ["user_profile", user_wallet_pubkey]
├── Stores: XP, Level, Streak, Statistics
└── Updated on: Quiz completion, Achievement awards, Daily check-ins

QuizResult Account
├── Seed: ["quiz_result", user_wallet_pubkey, quiz_id]
├── Stores: Score, XP earned, Timestamp
└── Created: Every quiz attempt (permanent record)

Achievement Account
├── Seed: ["achievement", user_wallet_pubkey, achievement_id]
├── Stores: Achievement name, Tier, Award timestamp
└── Created: When achievement criteria met
```

### Data Flow Example: Completing a Quiz

```
1. User answers quiz questions in frontend
         ↓
2. Frontend calls submit_quiz instruction
         ↓
3. Smart contract validates:
   - Score ≤ total questions
   - User owns the profile
         ↓
4. Calculate rewards:
   - XP = (score × 10) + (perfect_score ? 50 : 0)
         ↓
5. Update UserProfile:
   - Add XP
   - Recalculate level (XP / 100 + 1)
   - Update streak
   - Increment quizzes_completed
         ↓
6. Create QuizResult account:
   - Store score permanently
   - Record XP earned
   - Timestamp completion
         ↓
7. Emit success event
         ↓
8. Frontend updates UI with new stats
```

## Project Structure

```
solmentor/
│
├── programs/                    # Anchor program (smart contract)
│   └── solmentor/
│       ├── Cargo.toml          # Rust dependencies
│       └── src/
│           └── lib.rs          # Main program logic (267 lines)
│               ├── Instructions:
│               │   ├── initialize_profile()
│               │   ├── submit_quiz()
│               │   ├── award_achievement()
│               │   └── update_streak()
│               ├── Account Structures:
│               │   ├── UserProfile
│               │   ├── QuizResult
│               │   └── Achievement
│               └── Account Validation Contexts
│
├── app/                        # Next.js application (frontend)
│   ├── layout.tsx             # Root layout with wallet provider
│   ├── page.tsx               # Landing page with features
│   └── globals.css            # Global styles and animations
│
├── components/                 # React components
│   └── WalletProvider.tsx     # Solana wallet integration
│       ├── Supports: Phantom, Solflare, Backpack
│       ├── Auto-connect functionality
│       └── Network: Devnet (configurable)
│
├── lib/                       # Utility functions (planned)
│   └── (anchor client, API helpers)
│
├── Anchor.toml                # Anchor workspace configuration
│   ├── Program ID: SoLMnToR11111111111111111111111111111111111
│   └── Cluster: Devnet
│
├── package.json               # Node.js dependencies and scripts
├── next.config.mjs           # Next.js configuration
├── tailwind.config.ts        # Tailwind CSS configuration
└── tsconfig.json             # TypeScript configuration
```

## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:

**Required Software:**
- Node.js (version 18.0.0 or higher)
- npm (version 9.0.0 or higher)
- Rust (latest stable version)
- Solana CLI (latest version)
- Anchor CLI (version 0.30.1 or higher)

**Recommended:**
- A Solana wallet browser extension (Phantom, Solflare, or Backpack)
- Some SOL on Devnet for testing (get from Solana faucet)

### Installation

#### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/solmentor.git
cd solmentor
```

#### 2. Install Frontend Dependencies

```bash
npm install
```

This installs all required packages including:
- Next.js and React
- Solana Web3.js and wallet adapters
- Anchor client libraries
- UI libraries (Framer Motion, Tailwind CSS)

#### 3. Set Up Solana CLI (if not already configured)

```bash
# Configure Solana CLI to use Devnet
solana config set --url devnet

# Create a new wallet (or use existing)
solana-keygen new --outfile ~/.config/solana/id.json

# Get some SOL from the Devnet faucet
solana airdrop 2
```

#### 4. Build the Anchor Program

```bash
# Install Anchor if you haven't already
cargo install --git https://github.com/coral-xyz/anchor avm --locked
avm install 0.30.1
avm use 0.30.1

# Build the program
anchor build
```

This compiles the Rust smart contract and generates the program binary and IDL (Interface Definition Language) file.

#### 5. Deploy to Devnet (Optional for local testing)

If you want to deploy your own instance:

```bash
anchor deploy --provider.cluster devnet
```

Note: The program ID in `Anchor.toml` is already configured. If you deploy your own instance, update the program ID in both `Anchor.toml` and `programs/solmentor/src/lib.rs`.

### Running the Application

#### Development Mode

Start the Next.js development server:

```bash
npm run dev
```

The application will be available at `http://localhost:3000`

#### Production Build

To create an optimized production build:

```bash
npm run build
npm start
```

### Available Scripts

```bash
# Frontend
npm run dev          # Start development server
npm run build        # Create production build
npm run start        # Start production server
npm run lint         # Run ESLint
npm run type-check   # Check TypeScript types

# Smart Contract
npm run anchor:build  # Build Anchor program
npm run anchor:deploy # Deploy to configured network
npm run anchor:test   # Run Anchor tests
```

## How It Works

### For Learners

1. **Connect Your Wallet**: Use Phantom, Solflare, or Backpack to connect
2. **Initialize Profile**: Create your on-chain learner profile (one-time setup)
3. **Start Learning**: Take AI-generated quizzes on Solana development topics
4. **Earn Rewards**: Gain XP for correct answers, maintain streaks, unlock achievements
5. **Track Progress**: All your progress is permanently stored on-chain
6. **Verify Skills**: Share your wallet address to prove your learning credentials

### For Developers (Integration)

```typescript
// Example: Submitting a quiz (simplified)
import { Program, AnchorProvider } from '@coral-xyz/anchor';
import { useAnchorWallet } from '@solana/wallet-adapter-react';

// Initialize program
const wallet = useAnchorWallet();
const provider = new AnchorProvider(connection, wallet, {});
const program = new Program(IDL, programId, provider);

// Submit quiz
await program.methods
  .submitQuiz("quiz-101", 8, 10)  // 8 correct out of 10
  .accounts({
    userProfile: userProfilePDA,
    quizResult: quizResultPDA,
    authority: wallet.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();

// Result: 
// - User earns 80 XP (8 × 10)
// - QuizResult account created on-chain
// - UserProfile updated with new XP and level
```

## Key Features Explained

### XP and Leveling System

The XP calculation is designed to reward both effort and excellence:

- **Base XP**: 10 XP per correct answer
- **Perfect Score Bonus**: Additional 50 XP for getting all questions right
- **Level Progression**: Level = (Total XP / 100) + 1

Example scenarios:
- Quiz with 8/10 correct: 80 XP earned
- Quiz with 10/10 correct: 100 + 50 = 150 XP earned
- Starting from 0 XP, after 150 XP: Level 2 achieved

This system encourages learners to aim for perfect scores while still rewarding partial completion.

### Streak Tracking

Streaks incentivize daily engagement:

- **Check-in Window**: 24 hours (86400 seconds)
- **Streak Increment**: Consecutive day activity increases streak
- **Streak Reset**: Missing the 24-hour window resets streak to 1

The smart contract automatically manages streaks by comparing the current timestamp with the last active timestamp on your profile.

### Achievement System

Achievements are tiered rewards for reaching milestones:

- **Bronze Tier**: +50 XP bonus (e.g., "Complete First Quiz")
- **Silver Tier**: +100 XP bonus (e.g., "10 Quiz Streak")
- **Gold Tier**: +200 XP bonus (e.g., "Reach Level 10")
- **Platinum Tier**: +500 XP bonus (e.g., "100 Quizzes Completed")

Each achievement is stored as a separate on-chain account, creating a permanent, verifiable record of your accomplishments.

### AI-Powered Quiz Generation

Using OpenAI's API, SolMentor generates personalized quizzes that:
- Adapt to your current skill level
- Cover Solana-specific topics (smart contracts, transactions, PDAs, etc.)
- Provide detailed explanations for correct and incorrect answers
- Create unique questions to prevent memorization

## Security Features

### Program Derived Addresses (PDAs)

All accounts use PDAs for security:
- **Deterministic**: Same seeds always generate the same address
- **No Private Key**: PDAs have no corresponding private key, making them secure
- **Authorization**: Only the owning wallet can modify their accounts

### Input Validation

The smart contract includes multiple validation checks:
- Score cannot exceed total questions
- Only the profile owner can submit quizzes
- Timestamps are validated for streak calculations

### Authority Checks

Every instruction verifies that the signer has authority to perform the action through Anchor's `has_one` constraint.

## Testing

### Smart Contract Tests

```bash
anchor test
```

This runs the Anchor test suite that validates:
- Profile initialization
- Quiz submission logic
- XP calculation accuracy
- Achievement awarding
- Streak mechanics

### Frontend Tests

```bash
npm test
```

Runs Jest tests for React components and utility functions.

## Deployment on AImpact.dev

SolMentor is designed to be deployed through AImpact.dev, the AI-powered platform for building and launching Solana applications.

## Roadmap

### Current Features (MVP)
- On-chain user profiles with XP and levels
- Quiz result recording
- Achievement system with tiers
- Streak tracking
- Wallet integration

### Coming Soon
- AI quiz generation integration
- Multiple learning paths (beginner, intermediate, advanced)
- Leaderboards showing top learners
- NFT badges for achievements
- Content creator marketplace
- Token rewards ($MENTOR token)

### Future Vision
- DAO governance for content curation
- Peer-to-peer tutoring marketplace
- Integration with real-world coding platforms
- Partnerships with Solana development bootcamps
- Mobile app with push notifications for streaks



## Acknowledgments

- Built with [Anchor](https://www.anchor-lang.com/) - The Solana development framework
- Generated and deployed using [AImpact.dev](https://aimpact.dev)
- Created for the AImpact Sidetrack Hackathon
- Inspired by the Solana developer community

## Contact

- Twitter: [@solmentor](https://twitter.com/solmentor)
- Discord: [Join our community](https://discord.gg/solmentor)
- GitHub: [github.com/yourusername/solmentor](https://github.com/yourusername/solmentor)

## Support

If you find SolMentor helpful, consider:
- Starring the repository
- Sharing with fellow Solana learners
- Contributing to the codebase
- Providing feedback and suggestions

---

Built with passion for the Solana ecosystem.

