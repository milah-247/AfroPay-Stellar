import { Module } from '@nestjs/common';
import { AnchorService } from './anchor.service';
import { AnchorController } from './anchor.controller';

@Module({
  providers: [AnchorService],
  controllers: [AnchorController],
})
export class AnchorModule {}
