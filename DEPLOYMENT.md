# SolMentor Deployment Guide

This guide walks you through deploying SolMentor on AImpact.dev and preparing your submission for the AImpact Sidetrack Hackathon. We'll cover everything from building your Anchor program to creating your Twitter announcement.

## Table of Contents

1. [Pre-Deployment Checklist](#pre-deployment-checklist)
2. [Building the Anchor Program](#building-the-anchor-program)
3. [Deploying to Solana Devnet](#deploying-to-solana-devnet)
4. [Preparing the Frontend](#preparing-the-frontend)
5. [Deploying on AImpact.dev](#deploying-on-aimpactdev)
6. [Post-Deployment Testing](#post-deployment-testing)
7. [Hackathon Submission](#hackathon-submission)
8. [Troubleshooting](#troubleshooting)

## Pre-Deployment Checklist

Before deploying, ensure you have:

### Required Tools
- [ ] Node.js 18+ installed
- [ ] Rust and Cargo installed
- [ ] Solana CLI installed and configured
- [ ] Anchor CLI v0.30.1+ installed
- [ ] Git for version control
- [ ] A Solana wallet with Devnet SOL

### Required Accounts
- [ ] AImpact.dev account created
- [ ] Twitter (X) account for announcement
- [ ] Solana wallet (Phantom, Solflare, or Backpack)
- [ ] Discord account for AImpact community

### Funded Wallet
- [ ] At least 5 SOL on Devnet for deployment and testing

Get Devnet SOL:
```bash
solana config set --url devnet
solana airdrop 2
# Wait a few seconds, then:
solana airdrop 2
# Check balance:
solana balance
```

If the airdrop fails (rate limits), use the web faucet:
- https://faucet.solana.com/
- https://solfaucet.com/

## Building the Anchor Program

### Step 1: Configure Solana CLI

```bash
# Set to Devnet
solana config set --url devnet

# Verify configuration
solana config get

# Output should show:
# RPC URL: https://api.devnet.solana.com
```

### Step 2: Build the Program

```bash
# From project root
cd programs/solmentor

# Clean previous builds (if any)
anchor clean

# Build the program
anchor build
```

This process:
- Compiles Rust code to BPF bytecode
- Generates IDL (Interface Definition Language) file
- Creates the program binary

**Expected output**:
```
Compiling solmentor v0.1.0
Finished release [optimized] target(s) in 45.23s
```

The compiled program will be at:
- Binary: `target/deploy/solmentor.so`
- IDL: `target/idl/solmentor.json`

### Step 3: Verify Program ID

```bash
# Check the program ID in the binary
solana-keygen pubkey target/deploy/solmentor-keypair.json
```

This should match the ID in:
- `Anchor.toml`: `solmentor = "SoLMnToR11111111111111111111111111111111111"`
- `programs/solmentor/src/lib.rs`: `declare_id!("SoLMnToR11111111111111111111111111111111111");`

**If they don't match**, you have two options:

**Option A: Use the existing ID** (recommended for hackathon)
```bash
# Copy the existing keypair
cp path/to/existing/solmentor-keypair.json target/deploy/
```

**Option B: Update to new ID** (if starting fresh)
```bash
# Get the new program ID
NEW_ID=$(solana-keygen pubkey target/deploy/solmentor-keypair.json)

# Update Anchor.toml
sed -i '' "s/SoLMnToR11111111111111111111111111111111111/$NEW_ID/g" Anchor.toml

# Update lib.rs
sed -i '' "s/SoLMnToR11111111111111111111111111111111111/$NEW_ID/g" programs/solmentor/src/lib.rs

# Rebuild
anchor build
```

## Deploying to Solana Devnet

### Step 1: Deploy the Program

```bash
# Deploy to Devnet
anchor deploy --provider.cluster devnet
```

**What happens**:
1. Uploads the compiled program to Devnet
2. Allocates space for the program
3. Marks the program as executable
4. Costs approximately 2-3 SOL (refundable if you close the program)

**Expected output**:
```
Deploying cluster: https://api.devnet.solana.com
Upgrade authority: Your_Wallet_Address
Deploying program "solmentor"...
Program Id: SoLMnToR11111111111111111111111111111111111

Deploy success
```

### Step 2: Verify Deployment

```bash
# Check program account info
solana program show SoLMnToR11111111111111111111111111111111111

# Output shows:
# Program Id: SoLMnToR11111111111111111111111111111111111
# Owner: BPFLoaderUpgradeab1e11111111111111111111111
# ProgramData Address: ...
# Authority: Your_Wallet_Address
# Last Deployed In Slot: ...
# Data Length: ... bytes
```

**Check program is executable**:
```bash
solana account SoLMnToR11111111111111111111111111111111111

# Look for "executable: true"
```

### Step 3: Test the Program (Optional but Recommended)

```bash
# Run Anchor tests against Devnet
anchor test --skip-local-validator

# Or test individual instructions
npm run anchor:test
```

If tests pass, your program is working correctly on Devnet.

## Preparing the Frontend

### Step 1: Update Environment Variables

Create `.env.local` file in project root:

```bash
# Solana Network Configuration
NEXT_PUBLIC_SOLANA_NETWORK=devnet

# RPC Endpoint (use custom RPC for better performance)
# Default Devnet RPC: https://api.devnet.solana.com
# Or use Helius, QuickNode, etc.
NEXT_PUBLIC_RPC_ENDPOINT=https://api.devnet.solana.com

# Program ID (your deployed program)
NEXT_PUBLIC_PROGRAM_ID=SoLMnToR11111111111111111111111111111111111

# Optional: OpenAI API for quiz generation
OPENAI_API_KEY=your_openai_api_key_here

# Optional: Analytics
NEXT_PUBLIC_GA_ID=G-XXXXXXXXXX
```

**Security Note**: 
- Variables starting with `NEXT_PUBLIC_` are exposed to the browser
- Never put private keys or sensitive secrets in these variables
- For production, use environment-specific configs

### Step 2: Generate and Copy IDL

The IDL file is needed for the frontend to interact with your program:

```bash
# Copy IDL to frontend
mkdir -p lib/idl
cp target/idl/solmentor.json lib/idl/

# Or if you have a different structure:
cp target/idl/solmentor.json app/lib/idl/
```

**IDL contains**:
- Program ID
- Instruction definitions
- Account structures
- Type definitions

### Step 3: Build Frontend

```bash
# Install dependencies (if not already done)
npm install

# Type check
npm run type-check

# Build for production
npm run build
```

**Expected output**:
```
Route (app)                              Size     First Load JS
┌ ○ /                                   X kB           XX kB
└ ○ /[other routes]                     X kB           XX kB

○  (Static)  prerendered as static content
```

### Step 4: Test Locally

```bash
# Run production build locally
npm start

# Or run in development mode
npm run dev
```

Open `http://localhost:3000` and test:
- [ ] Wallet connection works
- [ ] All pages load correctly
- [ ] No console errors
- [ ] Responsive design works on mobile

## Deploying on AImpact.dev

AImpact.dev is an AI-powered platform for building and deploying Solana applications. Here's how to deploy SolMentor.

### Step 1: Access AImpact.dev

1. Go to https://aimpact.dev
2. Connect your wallet (the same wallet that deployed the program)
3. Sign in with your wallet signature

### Step 2: Create New Project

1. Click "New Project" or "Build App"
2. Choose "Import Existing Project" (since we have code)
3. Or choose "Start from Scratch" and paste your code

**Project Details**:
- **Name**: SolMentor
- **Description**: AI-Powered Decentralized Learning Platform on Solana
- **Category**: Education / DeFi / GameFi
- **Network**: Devnet

### Step 3: Upload or Connect Repository

**Option A: GitHub Integration** (Recommended)
1. Connect your GitHub account
2. Select your SolMentor repository
3. Choose the branch (usually `main`)
4. AImpact will auto-detect Next.js and Anchor

**Option B: Direct Upload**
1. Zip your project (excluding `node_modules` and `target`)
2. Upload the zip file
3. Wait for extraction and analysis

### Step 4: Configure Build Settings

AImpact should auto-detect, but verify:

**Smart Contract**:
- Framework: Anchor
- Version: 0.30.1
- Entry file: `programs/solmentor/src/lib.rs`
- Program ID: SoLMnToR11111111111111111111111111111111111

**Frontend**:
- Framework: Next.js 14
- Build command: `npm run build`
- Start command: `npm start`
- Environment variables: (paste from `.env.local`)

### Step 5: Deploy

1. Click "Deploy" button
2. AImpact will:
   - Install dependencies
   - Build Anchor program (if not already deployed)
   - Build Next.js frontend
   - Deploy to their hosting infrastructure
   - Assign a URL (e.g., `solmentor.aimpact.app`)

**Deployment time**: 3-10 minutes depending on project size.

### Step 6: Get Your Deployment URL

Once deployed, you'll receive:
- **App URL**: `https://solmentor.aimpact.app` (example)
- **Admin Dashboard**: For viewing logs and analytics
- **Deployment Status**: Build logs and error messages

**Save this URL** - you'll need it for your hackathon submission!

## Post-Deployment Testing

### Functionality Checklist

Visit your deployed app and test:

**Wallet Integration**:
- [ ] Connect wallet button works
- [ ] Wallet modal opens
- [ ] Can select different wallets
- [ ] Connection persists on refresh (autoconnect)
- [ ] Disconnect works

**UI/UX**:
- [ ] Landing page loads correctly
- [ ] All images and fonts load
- [ ] Animations work smoothly
- [ ] Responsive on mobile/tablet/desktop
- [ ] No console errors

**Smart Contract Interaction** (if implemented):
- [ ] Can initialize user profile
- [ ] Profile creation transaction succeeds
- [ ] Can submit quiz
- [ ] Quiz results are recorded on-chain
- [ ] XP and level update correctly
- [ ] Achievements can be awarded

**Performance**:
- [ ] Page load time < 3 seconds
- [ ] Lighthouse score > 80
- [ ] Works on slow connections

### Testing with Real Users

Ask 2-3 friends to test:
1. Send them the deployment URL
2. Have them connect their Devnet wallet
3. Walk through the user journey
4. Collect feedback on UX issues

**Common issues**:
- "I don't have a wallet" → Provide Phantom installation link
- "Transaction failed" → Check they have Devnet SOL
- "Page won't load" → Check deployment logs on AImpact

### Performance Testing

Use Lighthouse (Chrome DevTools):
```bash
# Install Lighthouse CLI
npm install -g lighthouse

# Run audit
lighthouse https://solmentor.aimpact.app --view
```

**Target scores**:
- Performance: 80+
- Accessibility: 90+
- Best Practices: 90+
- SEO: 80+

## Hackathon Submission

### Required: Twitter Post

Create a tweet announcing your project:

**Template**:
```
🎓 Introducing SolMentor - Learn Solana & Earn On-Chain Rewards!

Built for @aimpact_dev Sidetrack Hackathon 🚀

✅ AI-powered quizzes
✅ On-chain achievements
✅ Gamified learning with XP & levels
✅ Permanent progress on Solana

Try it: [your-aimpact-url]

#Solana #Web3 #Education
```

**Requirements**:
- [ ] Tag @aimpact_dev
- [ ] Subscribe to @aimpact_dev
- [ ] Include your deployment URL
- [ ] Use relevant hashtags
- [ ] Add screenshot or video (recommended)

**Tips for engagement**:
- Add a demo video or GIF
- Share your motivation for building
- Highlight unique features
- Ask for feedback

### Required: Devfolio/Hackathon Submission Form

Fill out the submission form with:

**Project Information**:
- **Project Name**: SolMentor
- **Tagline**: AI-Powered Decentralized Learning Platform on Solana
- **Description**: (150-300 words explaining the project)
- **Deployment URL**: Your AImpact.dev URL
- **Twitter Post**: Link to your tweet
- **Country/Location**: Your location or Superteam affiliation

**Technical Details**:
- **Blockchain**: Solana (Devnet)
- **Smart Contract Address**: SoLMnToR11111111111111111111111111111111111
- **GitHub Repository**: https://github.com/yourusername/solmentor
- **Tech Stack**: Anchor, Next.js, TypeScript, Tailwind CSS

**Description Example**:

"SolMentor is a gamified learning platform that brings educational progress on-chain. Unlike traditional e-learning platforms where your achievements are stored on centralized servers, SolMentor records every quiz, every XP point, and every milestone directly on the Solana blockchain.

The platform features an intelligent XP system (10 XP per correct answer + 50 bonus for perfect scores), progressive leveling (100 XP per level), daily streaks to encourage engagement, and tiered achievements (Bronze to Platinum) with bonus rewards. All of this is powered by an Anchor smart contract that ensures transparent, verifiable, and permanent records.

Built specifically for learning Solana development, SolMentor creates a feedback loop where learners interact with the very technology they're studying. The frontend uses Next.js 14 with full wallet integration (Phantom, Solflare, Backpack), and AI-powered quiz generation is planned for personalized learning paths.

SolMentor demonstrates how Web3 can transform education by providing true ownership of credentials, transparent progression systems, and meaningful incentives for daily learning."

### Bonus: Additional Submissions

To maximize your hackathon impact:

**Colosseum Cyberpunk**:
- [ ] Applied with SolMentor
- [ ] Mentioned AImpact in pitch deck
- [ ] Mentioned AImpact in video demo

**Launch Token on Heaven**:
- [ ] Created $MENTOR token via AImpact
- [ ] Used "Launch Token" button in AImpact header
- [ ] Integrated token rewards in roadmap

**Product Hunt Support**:
- [ ] Filled out support form: https://forms.gle/p35roGy4tTgphtDy9
- [ ] Ready to upvote when AImpact launches

**Legends.fun Support**:
- [ ] Supported with invite code: STKTDS
- [ ] Link: https://www.legends.fun/products/870efc22-80f4-498d-9720-34ea429288ab

### Documentation Links

Include these in your submission:
- README: Project overview and setup
- SMART_CONTRACT.md: Detailed smart contract explanation
- FRONTEND.md: Frontend architecture documentation
- ARCHITECTURE_DIAGRAMS.md: Visual system design

These show judges the depth of your work and make evaluation easier.

## Troubleshooting

### Common Deployment Issues

**Issue: "Program deployment failed"**

```bash
# Check wallet balance
solana balance

# If low, airdrop more
solana airdrop 2

# Retry deployment
anchor deploy --provider.cluster devnet
```

**Issue: "Transaction simulation failed"**

Possible causes:
- Incorrect program ID in code
- Account size too small
- Invalid instruction data

Solution:
```bash
# Rebuild with correct program ID
anchor build

# Verify IDL matches code
cat target/idl/solmentor.json

# Test locally first
anchor test --skip-local-validator
```

**Issue: "Cannot find program ID"**

Frontend can't find the deployed program.

Solution:
```bash
# Verify program is deployed
solana program show SoLMnToR11111111111111111111111111111111111

# Check NEXT_PUBLIC_PROGRAM_ID in .env.local
cat .env.local | grep PROGRAM_ID

# Rebuild frontend
npm run build
```

### Frontend Issues

**Issue: "Wallet not connecting"**

Checklist:
- [ ] Wallet extension installed
- [ ] On correct network (Devnet)
- [ ] Browser not blocking popups
- [ ] Tried refreshing page

**Issue: "Transaction failed"**

Common causes:
- Insufficient SOL for rent/fees
- Program account doesn't exist
- Wrong network (mainnet wallet on devnet app)

Solution:
```bash
# Check wallet network in extension
# Should show "Devnet"

# Get Devnet SOL
solana airdrop 2 YOUR_WALLET_ADDRESS --url devnet
```

**Issue: "Build failed on AImpact"**

Check AImpact deployment logs:
1. Go to project dashboard
2. Click "Deployment Logs"
3. Look for error messages

Common fixes:
- Add missing environment variables
- Fix TypeScript errors (`npm run type-check`)
- Add missing dependencies to `package.json`

### Performance Issues

**Issue: "Slow RPC calls"**

The public Devnet RPC can be slow or rate-limited.

Solution:
```bash
# Use a dedicated RPC provider
# Add to .env.local:
NEXT_PUBLIC_RPC_ENDPOINT=https://your-custom-rpc-endpoint

# Free options:
# - Helius: https://helius.dev
# - QuickNode: https://quicknode.com
# - Alchemy: https://alchemy.com
```

**Issue: "Page loads slowly"**

Optimize:
```bash
# Analyze bundle
npm run build -- --analyze

# Reduce dependencies
npm prune

# Optimize images
# Use next/image for all images
# Compress images before adding to project
```

## Post-Submission

### Marketing Your Project

After submitting:

1. **Share on Social Media**
   - Twitter thread explaining features
   - LinkedIn post for professional network
   - Reddit (r/solana, r/CryptoCurrency)
   - Discord communities (Solana, AImpact, Superteam)

2. **Create Demo Content**
   - Screen recording of user flow
   - Walkthrough video explaining smart contract
   - Blog post on dev.to or Medium

3. **Engage with Community**
   - Respond to comments and questions
   - Share updates and improvements
   - Thank supporters and testers

### Iteration Based on Feedback

Monitor:
- Twitter replies and DMs
- Discord questions in AImpact server
- GitHub issues (if repo is public)
- Direct user feedback

Quick wins:
- Fix critical bugs immediately
- Add requested features if feasible
- Improve documentation based on questions
- Update demo video if needed

### Preparing for Judging

Judges will evaluate:
- **Innovation**: What's unique about SolMentor?
- **Technical Execution**: Does it work? Is code quality good?
- **User Experience**: Is it easy to use?
- **Impact**: Does it solve a real problem?
- **Completeness**: Is it a working product or just a demo?

Make sure:
- [ ] Demo works flawlessly (test again before judging)
- [ ] Documentation is clear and complete
- [ ] Twitter post is live and engaging
- [ ] You can explain technical decisions
- [ ] You have a vision for the future

## Conclusion

Deploying on AImpact.dev and submitting to the hackathon is straightforward if you follow these steps:

1. Build and deploy Anchor program to Devnet
2. Prepare frontend with correct environment variables
3. Deploy on AImpact.dev platform
4. Test thoroughly
5. Create Twitter announcement
6. Submit to hackathon with all required info
7. Optionally pursue bonus points

Remember: The goal isn't just to submit, but to create something valuable that showcases your skills and contributes to the Solana ecosystem.

**Good luck with your submission! 🚀**

Need help? Join the AImpact Discord: https://discord.gg/MFTPPm3gwY

