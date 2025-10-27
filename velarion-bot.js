import {
  Client,
  GatewayIntentBits,
  Partials,
  EmbedBuilder,
  ActionRowBuilder,
  ButtonBuilder,
  ButtonStyle,
  StringSelectMenuBuilder,
} from "discord.js";
import dotenv from "dotenv";
dotenv.config();

const client = new Client({
  intents: [
    GatewayIntentBits.Guilds,
    GatewayIntentBits.GuildMembers,
    GatewayIntentBits.GuildMessages,
    GatewayIntentBits.MessageContent,
  ],
  partials: [Partials.Message, Partials.Channel, Partials.Reaction],
});

// 🔧 Настройки ID
const WELCOME_ROLE_ID = "1432073422149648394";
const VERIFIED_ROLE_ID = "1432073324216717452";
const WELCOME_CHANNEL_ID = "1432075319191408753";
const TICKET_CHANNEL_ID = "1432080991823990877";

// Вопросы для верификации
const questions = [
  { q: "1️⃣ What is Velarion?", options: ["A blockchain network","A game company","A social media platform","A payment app"], answer: 0 },
  { q: "2️⃣ Who governs the Velarion Blockchain?", options: ["The United Nations","ABM Foundation","Ethereum Foundation","No one, it’s centralized"], answer: 1 },
  { q: "3️⃣ What is the native token of Velarion?", options: ["VLR","VEL","VION","ABM"], answer: 0 },
  { q: "4️⃣ What are the main focuses of Velarion?", options: ["Gaming and memes","Security, scalability, interoperability","AI content creation","Private messaging"], answer: 1 },
  { q: "5️⃣ What does ABM Foundation stand for in Velarion’s mission?", options: ["To control the network privately","To build transparent, sustainable decentralized infrastructures","To create entertainment apps","To make money from ads"], answer: 1 },
];

// Быстрые вопросы и ответы для тикетов
const faqTickets = [
  { label: "What is Velarion?", answer: "A next-generation decentralized blockchain network powered by ABM Foundation." },
  { label: "Who governs Velarion?", answer: "ABM Foundation, a transparent and sustainable organization." },
  { label: "What is the native token?", answer: "VLR — used for transactions, validation, and ecosystem operations." },
  { label: "How can I create a Velarion wallet?", answer: "Use the official Velarion wallet app or supported third-party wallets." },
  { label: "My transaction is pending/failed.", answer: "Check network status and ensure enough gas; wait a few minutes." },
  { label: "How can I stake VLR?", answer: "Use the official staking platform." },
  { label: "What is Velarion’s consensus mechanism?", answer: "Scalable and secure Proof-of-Stake (PoS)." },
  { label: "How can I become a validator?", answer: "Stake the required amount of VLR and follow validator guide." },
  { label: "How to check network status?", answer: "Use the official Velarion explorer." },
  { label: "How can I join the Velarion community?", answer: "Join official Discord, Telegram, or social media." },
  { label: "Can I propose changes or improvements?", answer: "Yes, submit proposals via the governance portal." },
  { label: "How do I participate in governance voting?", answer: "Stake VLR and vote on proposals via governance interface." },
  { label: "How do I keep my wallet safe?", answer: "Never share private keys or seed phrases; use hardware wallets." },
  { label: "I think I’ve been scammed.", answer: "Report immediately to admins in Discord; do not send funds to unknown addresses." },
  { label: "Are there phishing websites?", answer: "Always use official Velarion links and double-check URLs." },
];

// Активные тесты и тикеты
const activeTests = new Map();
const activeTickets = new Map();

// 🔹 Когда бот запускается
client.once("ready", () => {
  console.log(`✅ Velarion bot is online as ${client.user.tag}`);

  // Устанавливаем био и активность
  client.user.setPresence({
    activities: [{ name: "!help" }],
    status: "online",
  });

  // Можно установить username и avatar (опционально)
  client.user.setUsername("Velarion Bot").catch(console.error);
  // client.user.setAvatar("https://i.imgur.com/youravatar.png"); // если хочешь поменять аватар
});

// 🔹 Приветствие новых участников
client.on("guildMemberAdd", async (member) => {
  try {
    const welcomeRole = member.guild.roles.cache.get(WELCOME_ROLE_ID);
    if (welcomeRole) await member.roles.add(welcomeRole);

    const channel = member.guild.channels.cache.get(WELCOME_CHANNEL_ID);
    if (channel) {
      const embed = new EmbedBuilder()
        .setTitle("🌌 Welcome to Velarion!")
        .setDescription(
          `Greetings, **${member.user.username}**! 👋\n\nWelcome to **Velarion**, the next-generation decentralized ecosystem powered by **ABM Foundation**.\n\nHere you can:\n🌐 Explore cutting-edge blockchain technology\n💬 Connect with pioneers in the Web3 revolution\n🚀 Contribute to sustainable and transparent innovation\n\n> To unlock all channels, please complete the verification test in **#verify**.\n\nWe’re thrilled to have you here — your journey in the Velarion Network begins now! 💎`
        )
        .setColor("#00ADEF")
        .setThumbnail(member.user.displayAvatarURL({ dynamic: true }))
        .setFooter({ text: "Velarion Blockchain — powered by ABM Foundation" })
        .setTimestamp();

      await channel.send({ content: `${member}`, embeds: [embed] });
    }
  } catch (error) {
    console.error("Error during welcome:", error);
  }
});

// 🔹 Команда !help
client.on("messageCreate", async (message) => {
  if (message.content === "!help") {
    const embed = new EmbedBuilder()
      .setTitle("📜 Velarion Bot Commands")
      .setColor("#00ADEF")
      .setDescription(
        "**!help** — Show this help message\n" +
        "**!create_verify** — Send verification start message with button\n" +
        "**!ticket** — Start a support ticket via DM\n"
      );
    await message.channel.send({ embeds: [embed] });
  }

  // Команда создания теста (для админов)
  if (message.content === "!create_verify") {
    if (!message.member.permissions.has("Administrator")) return;

    const button = new ButtonBuilder()
      .setCustomId("start_verification")
      .setLabel("Start Verification")
      .setStyle(ButtonStyle.Primary);

    const row = new ActionRowBuilder().addComponents(button);

    const embed = new EmbedBuilder()
      .setTitle("🛡️ Velarion Verification Portal")
      .setDescription(
        "Welcome! 🌍\n\nTo unlock full access to the Velarion community, complete the short verification test.\n\nClick **Start Verification** below to begin."
      )
      .setColor("#00ADEF")
      .setFooter({ text: "Velarion Blockchain — verification process" });

    await message.channel.send({ embeds: [embed], components: [row] });
  }

  // Команда тикета
  if (message.content === "!ticket") {
    const user = message.author;
    if (activeTickets.has(user.id)) {
      return message.reply("You already have an active ticket. Please finish it first.");
    }

    await message.reply("📩 Check your DMs to start the ticket process.");
    await user.createDM();

    // Создаём тикет с быстрыми вопросами
    const row = new ActionRowBuilder().addComponents(
      new StringSelectMenuBuilder()
        .setCustomId("faq_ticket_select")
        .setPlaceholder("Select your question...")
        .addOptions(faqTickets.map((q) => ({ label: q.label, value: q.label })))
    );

    await user.send({ content: "Select your question for support:", components: [row] });
    activeTickets.set(user.id, []);
  }
});

// 🔹 Тест и тикеты
client.on("interactionCreate", async (interaction) => {
  try {
    // Верификация: кнопка
    if (interaction.isButton() && interaction.customId === "start_verification") {
      const guild = client.guilds.cache.first();
      const member = guild.members.cache.get(interaction.user.id);

      if (member.roles.cache.has(VERIFIED_ROLE_ID)) {
        await interaction.reply({ content: "✅ You are already Verified!", ephemeral: true });
        return;
      }

      await interaction.reply({ content: "Verification started! Check your DMs 📩", ephemeral: true });
      const user = interaction.user;
      await user.createDM();
      activeTests.set(user.id, { step: 0, correct: 0 });
      await sendQuestion(user);
    }

    // Верификация select menu
    if (interaction.isStringSelectMenu() && interaction.customId === "answer_select") {
      const test = activeTests.get(interaction.user.id);
      if (!test) return;

      const selected = parseInt(interaction.values[0]);
      const current = questions[test.step];
      if (selected === current.answer) test.correct++;
      test.step++;

      if (test.step < questions.length) {
        await sendQuestion(interaction.user);
      } else {
        const guild = client.guilds.cache.first();
        const member = guild.members.cache.get(interaction.user.id);
        if (test.correct >= 3) {
          const verifiedRole = guild.roles.cache.get(VERIFIED_ROLE_ID);
          if (verifiedRole) await member.roles.add(verifiedRole);
          await interaction.reply({ content: `✅ You passed! ${test.correct}/${questions.length}`, ephemeral: true });
        } else {
          await interaction.reply({ content: `❌ Only ${test.correct}/${questions.length} correct. Try again later.`, ephemeral: true });
        }
        activeTests.delete(interaction.user.id);
      }
    }

    // Быстрые тикеты
    if (interaction.isStringSelectMenu() && interaction.customId === "faq_ticket_select") {
      const userAnswers = activeTickets.get(interaction.user.id);
      if (!userAnswers) return;

      const selectedLabel = interaction.values[0];
      const faq = faqTickets.find(f => f.label === selectedLabel);
      userAnswers.push({ question: faq.label, answer: faq.answer });

      await interaction.reply({ content: "✅ Your question has been submitted to support.", ephemeral: true });

      const ticketChannel = client.channels.cache.get(TICKET_CHANNEL_ID);
      if (ticketChannel) {
        const embed = new EmbedBuilder()
          .setTitle(`🎫 Ticket from ${interaction.user.tag}`)
          .setDescription(userAnswers.map(x => `**Q:** ${x.question}\n**A:** ${x.answer}`).join("\n\n"))
          .setColor("#00ADEF")
          .setFooter({ text: `User ID: ${interaction.user.id}` })
          .setTimestamp();

        await ticketChannel.send({ embeds: [embed] });
      }

      activeTickets.delete(interaction.user.id);
    }
  } catch(e) {
    console.error("Interaction error:", e);
  }
});

// 🔹 Функция отправки вопросов теста
async function sendQuestion(user) {
  const test = activeTests.get(user.id);
  const q = questions[test.step];
  const options = q.options.map((opt,index)=>({ label: opt, value:index.toString() }));
  const select = new StringSelectMenuBuilder().setCustomId("answer_select").setPlaceholder("Choose your answer...").addOptions(options);
  const row = new ActionRowBuilder().addComponents(select);
  const embed = new EmbedBuilder().setTitle(`Question ${test.step+1}`).setDescription(q.q).setColor("#00ADEF");
  await user.send({ embeds:[embed], components:[row] });
}

client.login(process.env.BOT_TOKEN);
