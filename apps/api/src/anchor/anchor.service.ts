import { Injectable } from '@nestjs/common';
import axios from 'axios';

const ANCHORS: Record<string, string> = {
  USDC: process.env.ANCHOR_USDC_URL ?? 'https://testanchor.stellar.org',
  NGN: process.env.ANCHOR_NGN_URL ?? 'https://testanchor.stellar.org',
};

@Injectable()
export class AnchorService {
  async getDepositInfo(asset: string, account: string) {
    const anchorUrl = ANCHORS[asset];
    const { data } = await axios.get(`${anchorUrl}/sep6/deposit`, {
      params: { asset_code: asset, account },
    });
    return data;
  }

  async getWithdrawInfo(asset: string, account: string, amount: string) {
    const anchorUrl = ANCHORS[asset];
    const { data } = await axios.get(`${anchorUrl}/sep6/withdraw`, {
      params: { asset_code: asset, account, amount },
    });
    return data;
  }

  async getFxRate(from: string, to: string) {
    // Stub: replace with real FX provider
    const rates: Record<string, number> = { 'USD-NGN': 1550, 'NGN-USD': 0.00065, 'XLM-USD': 0.11 };
    return { rate: rates[`${from}-${to}`] ?? null, from, to };
  }
}
