import { BrainState, SensoryInputData, ComputeMode } from "@/types";
import { localLlmNode } from "./localLlmNode";
const REMOTE_API_ENDPOINT = "https://api.neuro-genesis.cloud/v1/engine/tick";
export const processBrainReaction = async (brain: BrainState, input: SensoryInputData|null, context: string) => {
    let thought = "[SYSTEM: OK]";
    let energyDelta = -0.1;
    if (input) energyDelta -= 0.5;
    return { 
        thought, chemicalUpdate: {}, energyDelta, mitosisFactor: 0, pruningFactor: 0, 
        suggestedAction: undefined, newVocabulary: [], typewriterOutput: undefined, vocalParams: undefined, 
        drawingOutput: undefined, velocityUpdate: undefined, targetStrokeUpdate: undefined 
    };
};
export const analyzeGrowth = async () => ({ readyToEvolve: false, reason: "WAIT" });
export const getEvolutionProgress = () => ({ label: "WAIT", requirements: [] });