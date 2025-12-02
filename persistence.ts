import { BrainState, LifeStage } from '@/types';
    export const getFreshBrainState = (): BrainState => ({ stage: LifeStage.PROGENITORE, neuronCount: 1, energy: 100 } as any);
    export const loadBrainState = async () => null;
    export const saveBrainState = async (s:any) => {};
    export const clearBrainData = async () => {};
    export const sanitizeState = (s:any) => s;
    export const saveBrainStateSync = (s:any) => {};