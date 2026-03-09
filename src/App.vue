<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import StatusBadge from "./components/StatusBadge.vue";

type HealthTone = "ok" | "warn" | "danger" | "neutral";

interface Overview {
  platform: string;
  platform_label: string;
  machine_name: string;
  version: string;
  shell: string;
  recommended_channel: string;
  ready_for_install: boolean;
  checks: Array<{
    key: string;
    title: string;
    detail: string;
    tone: HealthTone;
  }>;
}

interface InstallPlan {
  headline: string;
  summary: string;
  steps: Array<{
    key: string;
    title: string;
    description: string;
    action: string;
    caution?: string;
  }>;
}

const overview = ref<Overview | null>(null);
const installPlan = ref<InstallPlan | null>(null);
const loading = ref(true);
const activeLog = ref("准备开始检测环境…");
const running = ref(false);
const lastResult = ref<string>("");

const heroLabel = computed(() => {
  if (!overview.value) {
    return "检测中";
  }

  return overview.value.ready_for_install ? "可直接安装" : "需要先修复环境";
});

async function loadContext() {
  loading.value = true;
  lastResult.value = "";

  try {
    const [systemOverview, plan] = await Promise.all([
      invoke<Overview>("get_system_overview"),
      invoke<InstallPlan>("get_install_plan")
    ]);

    overview.value = systemOverview;
    installPlan.value = plan;
    activeLog.value = "检测完成，可以开始执行安装或修复。";
  } catch (error) {
    activeLog.value = `读取环境信息失败：${String(error)}`;
  } finally {
    loading.value = false;
  }
}

async function runAction(action: "detect" | "install" | "repair") {
  running.value = true;
  activeLog.value =
    action === "detect"
      ? "正在重新检测环境…"
      : action === "install"
        ? "正在执行安装脚本…"
        : "正在执行修复脚本…";

  try {
    const result = await invoke<string>("run_installer_action", { action });
    lastResult.value = result;
    activeLog.value = result;
    await loadContext();
  } catch (error) {
    lastResult.value = `执行失败：${String(error)}`;
    activeLog.value = lastResult.value;
  } finally {
    running.value = false;
  }
}

onMounted(() => {
  loadContext();
});
</script>

<template>
  <main class="shell">
    <section class="hero">
      <div class="hero-copy">
        <p class="eyebrow">云乐AI · yuleai.com</p>
        <h1>OpenClaw 中文安装助手，安装更省心，上手更简单</h1>
        <p class="lead">
          这是云乐AI为中文用户整理的 OpenClaw 安装助手。Windows 走官方推荐的 WSL2 路线，macOS 走本机安装路线，把检测、修复、安装和日志统一收口到一个中文界面里。
        </p>
        <div class="brand-strip">
          <div class="brand-card">
            <p class="brand-title">云乐AI</p>
            <p class="brand-text">更强模型，更低价格，更易落地</p>
          </div>
          <div class="brand-card">
            <p class="brand-title">平台官网</p>
            <p class="brand-text">yuleai.com</p>
          </div>
        </div>
        <p class="lead lead-compact">
          致力于为开发者提供快速、便捷的 Web API 接口调用方案，打造稳定且易于使用的 API 接口平台，一站式集成几乎所有 AI 大模型。
        </p>
        <div class="hero-meta">
          <StatusBadge tone="neutral" :text="heroLabel" />
          <span v-if="overview">{{ overview.platform_label }} / {{ overview.version }}</span>
          <span v-if="overview">设备名：{{ overview.machine_name }}</span>
        </div>
      </div>
      <div class="hero-card">
        <p class="card-title">当前状态</p>
        <p class="card-highlight">{{ loading ? "环境检测中" : activeLog }}</p>
        <div class="actions">
          <button :disabled="running" @click="runAction('install')">一键安装</button>
          <button class="ghost" :disabled="running" @click="runAction('repair')">一键修复</button>
          <button class="ghost" :disabled="running" @click="runAction('detect')">重新检测</button>
        </div>
      </div>
    </section>

    <section class="grid">
      <article class="panel">
        <div class="panel-head">
          <h2>环境检测</h2>
          <StatusBadge
            :tone="overview?.ready_for_install ? 'ok' : 'warn'"
            :text="overview?.ready_for_install ? '满足安装条件' : '建议先修复'"
          />
        </div>
        <div v-if="overview" class="check-list">
          <div v-for="item in overview.checks" :key="item.key" class="check-item">
            <div>
              <p class="check-title">{{ item.title }}</p>
              <p class="check-detail">{{ item.detail }}</p>
            </div>
            <StatusBadge :tone="item.tone" :text="item.tone === 'ok' ? '正常' : item.tone === 'warn' ? '注意' : item.tone === 'danger' ? '异常' : '信息'" />
          </div>
        </div>
      </article>

      <article class="panel">
        <div class="panel-head">
          <h2>安装流程</h2>
          <span class="muted">{{ installPlan?.headline }}</span>
        </div>
        <div v-if="installPlan" class="step-list">
          <div v-for="step in installPlan.steps" :key="step.key" class="step-item">
            <p class="step-title">{{ step.title }}</p>
            <p class="step-description">{{ step.description }}</p>
            <p class="step-action">执行方式：{{ step.action }}</p>
            <p v-if="step.caution" class="step-caution">{{ step.caution }}</p>
          </div>
        </div>
      </article>
    </section>

    <section class="bottom-grid">
      <article class="panel">
        <div class="panel-head">
          <h2>云乐AI 服务信息</h2>
        </div>
        <ul class="plain-list">
          <li>官网：`yuleai.com`</li>
          <li>定位：更强模型，更低价格，更易落地。</li>
          <li>能力：为开发者提供快速、便捷的 Web API 接口调用方案，一站式集成几乎所有 AI 大模型。</li>
          <li>微信咨询：`Snake2118` / `Hua666ak`</li>
        </ul>
      </article>

      <article class="panel accent">
        <div class="panel-head">
          <h2>联系与执行结果</h2>
        </div>
        <div class="contact-box">
          <p>商务与技术咨询请联系微信：</p>
          <p><strong>Snake2118</strong></p>
          <p><strong>Hua666ak</strong></p>
        </div>
        <pre>{{ lastResult || "还没有执行任何操作。" }}</pre>
      </article>
    </section>
  </main>
</template>

<style scoped>
:global(body) {
  margin: 0;
  min-width: 320px;
  min-height: 100vh;
  background:
    radial-gradient(circle at top left, rgba(255, 225, 163, 0.55), transparent 25%),
    radial-gradient(circle at 85% 15%, rgba(103, 165, 255, 0.18), transparent 20%),
    linear-gradient(180deg, #fbf6ea 0%, #f5efe2 45%, #efe7d7 100%);
  color: #1f252b;
  font-family:
    "PingFang SC",
    "Noto Sans SC",
    "Microsoft YaHei",
    sans-serif;
}

:global(*) {
  box-sizing: border-box;
}

.shell {
  max-width: 1240px;
  margin: 0 auto;
  padding: 40px 24px 56px;
}

.hero {
  display: grid;
  grid-template-columns: 1.6fr 1fr;
  gap: 24px;
  align-items: stretch;
}

.hero-copy,
.hero-card,
.panel {
  border: 1px solid rgba(97, 72, 24, 0.12);
  background: rgba(255, 250, 240, 0.72);
  backdrop-filter: blur(16px);
  border-radius: 28px;
  box-shadow: 0 20px 60px rgba(91, 66, 25, 0.08);
}

.hero-copy {
  padding: 36px;
}

.eyebrow {
  margin: 0 0 12px;
  color: #8b5b1f;
  font-weight: 700;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  font-size: 0.8rem;
}

h1,
h2,
.card-highlight,
.step-title,
.check-title {
  font-family: "STSong", "Songti SC", serif;
}

h1 {
  margin: 0;
  font-size: clamp(2rem, 4vw, 4.2rem);
  line-height: 1.04;
  color: #1f2730;
}

.lead {
  max-width: 52rem;
  margin: 18px 0 0;
  font-size: 1.05rem;
  line-height: 1.8;
  color: #405060;
}

.lead-compact {
  max-width: 46rem;
  margin-top: 16px;
  font-size: 0.98rem;
}

.brand-strip {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 220px));
  gap: 14px;
  margin-top: 24px;
}

.brand-card {
  padding: 16px 18px;
  border-radius: 20px;
  background: rgba(255, 255, 255, 0.62);
  border: 1px solid rgba(139, 91, 31, 0.14);
}

.brand-title {
  margin: 0;
  color: #8b5b1f;
  font-size: 0.85rem;
  font-weight: 700;
  letter-spacing: 0.06em;
  text-transform: uppercase;
}

.brand-text {
  margin: 8px 0 0;
  color: #24313d;
  font-size: 1rem;
  line-height: 1.6;
}

.hero-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 10px 16px;
  margin-top: 22px;
  color: #5b6774;
  font-size: 0.95rem;
}

.hero-card {
  padding: 28px;
  background:
    linear-gradient(180deg, rgba(255, 245, 221, 0.95), rgba(255, 253, 248, 0.9)),
    linear-gradient(135deg, rgba(196, 139, 42, 0.16), rgba(29, 88, 143, 0.04));
}

.card-title {
  margin: 0;
  color: #7a4f1d;
  font-weight: 700;
}

.card-highlight {
  min-height: 7rem;
  margin: 16px 0 0;
  font-size: 1.6rem;
  line-height: 1.45;
  color: #213043;
}

.actions {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
  margin-top: 24px;
}

button {
  border: none;
  border-radius: 999px;
  padding: 14px 22px;
  background: linear-gradient(135deg, #b96c16, #d48f2b);
  color: #fffdf8;
  cursor: pointer;
  font-size: 0.96rem;
  font-weight: 700;
  transition:
    transform 160ms ease,
    box-shadow 160ms ease,
    opacity 160ms ease;
  box-shadow: 0 14px 30px rgba(177, 110, 25, 0.22);
}

button:hover:enabled {
  transform: translateY(-1px);
}

button:disabled {
  cursor: not-allowed;
  opacity: 0.55;
}

button.ghost {
  background: rgba(255, 250, 240, 0.8);
  color: #6e4d1b;
  box-shadow: inset 0 0 0 1px rgba(110, 77, 27, 0.16);
}

.grid,
.bottom-grid {
  display: grid;
  gap: 24px;
  margin-top: 24px;
}

.grid {
  grid-template-columns: repeat(2, minmax(0, 1fr));
}

.bottom-grid {
  grid-template-columns: 1.2fr 1fr;
}

.panel {
  padding: 26px;
}

.panel-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

h2 {
  margin: 0;
  font-size: 1.45rem;
  color: #26313d;
}

.muted {
  color: #647282;
  font-size: 0.92rem;
}

.check-list,
.step-list {
  display: grid;
  gap: 14px;
  margin-top: 18px;
}

.check-item,
.step-item {
  display: flex;
  justify-content: space-between;
  gap: 18px;
  padding: 16px 18px;
  border-radius: 20px;
  background: rgba(255, 255, 255, 0.58);
  border: 1px solid rgba(72, 83, 96, 0.08);
}

.step-item {
  flex-direction: column;
}

.check-title,
.step-title {
  margin: 0;
  font-size: 1.1rem;
  color: #24313d;
}

.check-detail,
.step-description,
.step-action,
.step-caution {
  margin: 6px 0 0;
  color: #51606f;
  line-height: 1.6;
}

.step-caution {
  color: #8b4c00;
}

.plain-list {
  margin: 18px 0 0;
  padding-left: 1.2rem;
  color: #44515f;
  line-height: 1.8;
}

.accent {
  background:
    linear-gradient(180deg, rgba(255, 255, 255, 0.8), rgba(245, 248, 251, 0.86)),
    linear-gradient(135deg, rgba(49, 116, 171, 0.05), rgba(194, 140, 48, 0.08));
}

pre {
  margin: 18px 0 0;
  white-space: pre-wrap;
  word-break: break-word;
  font-family:
    "SF Mono",
    "JetBrains Mono",
    monospace;
  color: #24415a;
  line-height: 1.65;
}

.contact-box {
  margin-top: 18px;
  padding: 16px 18px;
  border-radius: 20px;
  background: rgba(255, 255, 255, 0.62);
  color: #334a60;
  line-height: 1.7;
}

.contact-box p {
  margin: 0;
}

.contact-box p + p {
  margin-top: 4px;
}

@media (max-width: 960px) {
  .hero,
  .grid,
  .bottom-grid {
    grid-template-columns: 1fr;
  }

  .shell {
    padding: 24px 16px 40px;
  }

  .hero-copy,
  .hero-card,
  .panel {
    border-radius: 22px;
  }

  .brand-strip {
    grid-template-columns: 1fr;
  }
}
</style>
