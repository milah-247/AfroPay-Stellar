import { WalletService } from './wallet.service';

// Minimal unit tests without DB — test encryption helpers via reflection
describe('WalletService encryption', () => {
  let service: WalletService;

  beforeEach(() => {
    process.env.ENCRYPTION_KEY = 'a'.repeat(64); // 32-byte hex
    service = new WalletService(null as any);
  });

  it('encrypts and decrypts a secret key', () => {
    const secret = 'SCZANGBA5YHTNYVSK3TZQOZ6PFPAXDHDWZOBENXVGHD';
    const encrypted = (service as any).encrypt(secret);
    const decrypted = (service as any).decrypt(encrypted);
    expect(decrypted).toBe(secret);
  });

  it('produces different ciphertext each call (random IV)', () => {
    const secret = 'SCZANGBA5YHTNYVSK3TZQOZ6PFPAXDHDWZOBENXVGHD';
    const enc1 = (service as any).encrypt(secret);
    const enc2 = (service as any).encrypt(secret);
    expect(enc1).not.toBe(enc2);
  });
});
