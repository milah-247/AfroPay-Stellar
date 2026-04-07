import { AnchorService } from './anchor.service';

describe('AnchorService', () => {
  let service: AnchorService;

  beforeEach(() => { service = new AnchorService(); });

  it('returns a known FX rate for USD-NGN', async () => {
    const result = await service.getFxRate('USD', 'NGN');
    expect(result.rate).toBe(1550);
    expect(result.from).toBe('USD');
    expect(result.to).toBe('NGN');
  });

  it('returns null for unknown pair', async () => {
    const result = await service.getFxRate('EUR', 'JPY');
    expect(result.rate).toBeNull();
  });
});
