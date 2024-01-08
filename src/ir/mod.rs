
pub mod builder;
pub mod context;
pub mod pass_manager;
pub mod types;
pub mod consts;
pub mod structs;
pub mod value;
pub mod metadata;
pub mod module;
pub mod function;
pub mod basic_block;
pub mod memory_buffer;
pub mod intrinsic;



/*

    LLVMAddAttributeAtIndex⚠
    LLVMAddCallSiteAttribute⚠
    LLVMAddCase⚠
    LLVMAddClause⚠
    LLVMAddDestination⚠
    LLVMAddHandler⚠
    LLVMAddIncoming⚠
    LLVMAddTargetDependentFunctionAttr⚠

    LLVMAliasGetAliasee⚠
    LLVMAliasSetAliasee⚠

    LLVMAlignOf⚠



    LLVMCountIncoming⚠

    LLVMCreateMessage⚠
    LLVMDisposeMessage⚠

    LLVMDeleteGlobal⚠



    LLVMGetArrayLength⚠

    LLVMGetCallSiteAttributeCount⚠
    LLVMGetCallSiteAttributes⚠
    LLVMGetCallSiteEnumAttribute⚠
    LLVMGetCallSiteStringAttribute⚠
    LLVMGetCalledFunctionType⚠
    LLVMGetCalledValue⚠
    LLVMGetClause⚠
    LLVMGetCmpXchgFailureOrdering⚠
    LLVMGetCmpXchgSuccessOrdering⚠
    LLVMGetCondition⚠
    LLVMGetConstOpcode⚠
    LLVMGetCurrentDebugLocation⚠
    LLVMGetCurrentDebugLocation2⚠
    LLVMGetDLLStorageClass⚠
    LLVMGetDataLayoutStr⚠
    LLVMGetDebugLocColumn⚠
    LLVMGetDebugLocDirectory⚠
    LLVMGetDebugLocFilename⚠
    LLVMGetDebugLocLine⚠
    LLVMGetDiagInfoDescription⚠
    LLVMGetDiagInfoSeverity⚠
    LLVMGetElementAsConstant⚠
    LLVMGetElementType⚠
    LLVMGetEntryBasicBlock⚠
    LLVMGetEnumAttributeAtIndex⚠
    LLVMGetEnumAttributeKind⚠
    LLVMGetEnumAttributeKindForName⚠
    LLVMGetEnumAttributeValue⚠
    LLVMGetFCmpPredicate⚠
    LLVMGetFirstBasicBlock⚠
    LLVMGetFirstFunction⚠
    LLVMGetFirstGlobal⚠
    LLVMGetFirstGlobalAlias⚠
    LLVMGetFirstGlobalIFunc⚠
    LLVMGetFirstInstruction⚠
    LLVMGetFirstNamedMetadata⚠
    LLVMGetFirstParam⚠
    LLVMGetFirstUse⚠
    LLVMGetFunctionCallConv⚠
    LLVMGetGC⚠
    LLVMGetGlobalContext⚠
    LLVMGetGlobalIFuncResolver⚠
    LLVMGetGlobalParent⚠
    LLVMGetGlobalPassRegistry⚠
    LLVMGetHandlers⚠
    LLVMGetICmpPredicate⚠
    LLVMGetIncomingBlock⚠
    LLVMGetIncomingValue⚠
    LLVMGetIndices⚠
    LLVMGetInitializer⚠
    LLVMGetInlineAsm⚠
    LLVMGetInsertBlock⚠
    LLVMGetInstructionCallConv⚠
    LLVMGetInstructionOpcode⚠
    LLVMGetInstructionParent⚠
    LLVMGetIntTypeWidth⚠
    LLVMGetIntrinsicDeclaration⚠
    LLVMGetIntrinsicID⚠
    LLVMGetLastBasicBlock⚠
    LLVMGetLastEnumAttributeKind⚠
    LLVMGetLastFunction⚠
    LLVMGetLastGlobal⚠
    LLVMGetLastGlobalAlias⚠
    LLVMGetLastGlobalIFunc⚠
    LLVMGetLastInstruction⚠
    LLVMGetLastNamedMetadata⚠
    LLVMGetLastParam⚠
    LLVMGetLinkage⚠
    LLVMGetMDKindID⚠
    LLVMGetMDKindIDInContext⚠
    LLVMGetMDNodeNumOperands⚠
    LLVMGetMDNodeOperands⚠
    LLVMGetMDString⚠
    LLVMGetMaskValue⚠
    LLVMGetMetadata⚠
    LLVMGetNamedFunction⚠
    LLVMGetNamedGlobal⚠
    LLVMGetNamedGlobalAlias⚠
    LLVMGetNamedGlobalIFunc⚠
    LLVMGetNextBasicBlock⚠
    LLVMGetNextFunction⚠
    LLVMGetNextGlobal⚠
    LLVMGetNextGlobalAlias⚠
    LLVMGetNextGlobalIFunc⚠
    LLVMGetNextInstruction⚠
    LLVMGetNextNamedMetadata⚠
    LLVMGetNextParam⚠
    LLVMGetNextUse⚠
    LLVMGetNormalDest⚠
    LLVMGetNumArgOperands⚠
    LLVMGetNumClauses⚠
    LLVMGetNumContainedTypes⚠
    LLVMGetNumHandlers⚠
    LLVMGetNumIndices⚠
    LLVMGetNumMaskElements⚠
    LLVMGetNumOperands⚠
    LLVMGetNumSuccessors⚠
    LLVMGetOperand⚠
    LLVMGetOperandUse⚠
    LLVMGetOrInsertNamedMetadata⚠
    LLVMGetOrdering⚠
    LLVMGetParam⚠
    LLVMGetParamParent⚠
    LLVMGetParamTypes⚠
    LLVMGetParams⚠
    LLVMGetParentCatchSwitch⚠
    LLVMGetPersonalityFn⚠
    LLVMGetPointerAddressSpace⚠
    LLVMGetPoison⚠
    LLVMGetPreviousBasicBlock⚠
    LLVMGetPreviousFunction⚠
    LLVMGetPreviousGlobal⚠
    LLVMGetPreviousGlobalAlias⚠
    LLVMGetPreviousGlobalIFunc⚠
    LLVMGetPreviousInstruction⚠
    LLVMGetPreviousNamedMetadata⚠
    LLVMGetPreviousParam⚠
    LLVMGetReturnType⚠
    LLVMGetSection⚠
    LLVMGetSourceFileName⚠
    LLVMGetStringAttributeAtIndex⚠
    LLVMGetStringAttributeKind⚠
    LLVMGetStringAttributeValue⚠
    LLVMGetSubtypes⚠
    LLVMGetSuccessor⚠
    LLVMGetSwitchDefaultDest⚠
    LLVMGetTarget⚠
    LLVMGetThreadLocalMode⚠
    LLVMGetUndef⚠
    LLVMGetUndefMaskElem⚠
    LLVMGetUnnamedAddress⚠
    LLVMGetUnwindDest⚠
    LLVMGetUsedValue⚠
    LLVMGetUser⚠
    LLVMGetVectorSize⚠
    LLVMGetVisibility⚠
    LLVMGetVolatile⚠
    LLVMGetWeak⚠

    LLVMHasPersonalityFn⚠
    LLVMHasUnnamedAddr⚠Deprecated


    LLVMInsertBasicBlock⚠
    LLVMInsertBasicBlockInContext⚠
    LLVMInsertExistingBasicBlockAfterInsertBlock⚠

    LLVMInstructionClone⚠
    LLVMInstructionEraseFromParent⚠
    LLVMInstructionGetAllMetadataOtherThanDebugLoc⚠
    LLVMInstructionRemoveFromParent⚠

    How do these work?

    LLVMIsAAddrSpaceCastInst⚠
    LLVMIsAAllocaInst⚠
    LLVMIsAArgument⚠
    LLVMIsAAtomicCmpXchgInst⚠
    LLVMIsAAtomicRMWInst⚠
    LLVMIsABasicBlock⚠
    LLVMIsABinaryOperator⚠
    LLVMIsABitCastInst⚠
    LLVMIsABlockAddress⚠
    LLVMIsABranchInst⚠
    LLVMIsACallBrInst⚠
    LLVMIsACallInst⚠
    LLVMIsACastInst⚠
    LLVMIsACatchPadInst⚠
    LLVMIsACatchReturnInst⚠
    LLVMIsACatchSwitchInst⚠
    LLVMIsACleanupPadInst⚠
    LLVMIsACleanupReturnInst⚠
    LLVMIsACmpInst⚠
    LLVMIsAConstant⚠
    LLVMIsAConstantAggregateZero⚠
    LLVMIsAConstantArray⚠
    LLVMIsAConstantDataArray⚠
    LLVMIsAConstantDataSequential⚠
    LLVMIsAConstantDataVector⚠
    LLVMIsAConstantExpr⚠
    LLVMIsAConstantFP⚠
    LLVMIsAConstantInt⚠
    LLVMIsAConstantPointerNull⚠
    LLVMIsAConstantStruct⚠
    LLVMIsAConstantTokenNone⚠
    LLVMIsAConstantVector⚠
    LLVMIsADbgDeclareInst⚠
    LLVMIsADbgInfoIntrinsic⚠
    LLVMIsADbgLabelInst⚠
    LLVMIsADbgVariableIntrinsic⚠
    LLVMIsAExtractElementInst⚠
    LLVMIsAExtractValueInst⚠
    LLVMIsAFCmpInst⚠
    LLVMIsAFPExtInst⚠
    LLVMIsAFPToSIInst⚠
    LLVMIsAFPToUIInst⚠
    LLVMIsAFPTruncInst⚠
    LLVMIsAFenceInst⚠
    LLVMIsAFreezeInst⚠
    LLVMIsAFuncletPadInst⚠
    LLVMIsAFunction⚠
    LLVMIsAGetElementPtrInst⚠
    LLVMIsAGlobalAlias⚠
    LLVMIsAGlobalIFunc⚠
    LLVMIsAGlobalObject⚠
    LLVMIsAGlobalValue⚠
    LLVMIsAGlobalVariable⚠
    LLVMIsAICmpInst⚠
    LLVMIsAIndirectBrInst⚠
    LLVMIsAInlineAsm⚠
    LLVMIsAInsertElementInst⚠
    LLVMIsAInsertValueInst⚠
    LLVMIsAInstruction⚠
    LLVMIsAIntToPtrInst⚠
    LLVMIsAIntrinsicInst⚠
    LLVMIsAInvokeInst⚠
    LLVMIsALandingPadInst⚠
    LLVMIsALoadInst⚠
    LLVMIsAMDNode⚠
    LLVMIsAMDString⚠
    LLVMIsAMemCpyInst⚠
    LLVMIsAMemIntrinsic⚠
    LLVMIsAMemMoveInst⚠
    LLVMIsAMemSetInst⚠
    LLVMIsAPHINode⚠
    LLVMIsAPoisonValue⚠
    LLVMIsAPtrToIntInst⚠
    LLVMIsAResumeInst⚠
    LLVMIsAReturnInst⚠
    LLVMIsASExtInst⚠
    LLVMIsASIToFPInst⚠
    LLVMIsASelectInst⚠
    LLVMIsAShuffleVectorInst⚠
    LLVMIsAStoreInst⚠
    LLVMIsASwitchInst⚠
    LLVMIsATerminatorInst⚠
    LLVMIsATruncInst⚠
    LLVMIsAUIToFPInst⚠
    LLVMIsAUnaryInstruction⚠
    LLVMIsAUnaryOperator⚠
    LLVMIsAUndefValue⚠
    LLVMIsAUnreachableInst⚠
    LLVMIsAUser⚠
    LLVMIsAVAArgInst⚠
    LLVMIsAZExtInst⚠
    LLVMIsAtomicSingleThread⚠
    LLVMIsCleanup⚠
    LLVMIsConditional⚠
    LLVMIsConstant⚠
    LLVMIsConstantString⚠
    LLVMIsDeclaration⚠
    LLVMIsEnumAttribute⚠
    LLVMIsExternallyInitialized⚠
    LLVMIsFunctionVarArg⚠
    LLVMIsGlobalConstant⚠
    LLVMIsInBounds⚠
    LLVMIsLiteralStruct⚠
    LLVMIsMultithreaded⚠
    LLVMIsNull⚠
    LLVMIsOpaqueStruct⚠
    LLVMIsPackedStruct⚠
    LLVMIsPoison⚠
    LLVMIsStringAttribute⚠
    LLVMIsTailCall⚠
    LLVMIsThreadLocal⚠
    LLVMIsTypeAttribute⚠
    LLVMIsUndef⚠

    LLVMLabelType⚠
    LLVMLabelTypeInContext⚠




    LLVMRemoveCallSiteEnumAttribute⚠
    LLVMRemoveCallSiteStringAttribute⚠
    LLVMRemoveEnumAttributeAtIndex⚠
    LLVMRemoveGlobalIFunc⚠
    LLVMRemoveStringAttributeAtIndex⚠

    LLVMReplaceAllUsesWith⚠



    LLVMSetAlignment⚠
    LLVMSetArgOperand⚠
    LLVMSetAtomicRMWBinOp⚠
    LLVMSetAtomicSingleThread⚠
    LLVMSetCleanup⚠
    LLVMSetCmpXchgFailureOrdering⚠
    LLVMSetCmpXchgSuccessOrdering⚠
    LLVMSetCondition⚠
    LLVMSetCurrentDebugLocation⚠Deprecated
    LLVMSetCurrentDebugLocation2⚠
    LLVMSetDLLStorageClass⚠
    LLVMSetDataLayout⚠
    LLVMSetExternallyInitialized⚠
    LLVMSetFunctionCallConv⚠
    LLVMSetGC⚠
    LLVMSetGlobalConstant⚠
    LLVMSetGlobalIFuncResolver⚠
    LLVMSetInitializer⚠
    LLVMSetInstDebugLocation⚠
    LLVMSetInstrParamAlignment⚠
    LLVMSetInstructionCallConv⚠
    LLVMSetIsInBounds⚠
    LLVMSetLinkage⚠
    LLVMSetMetadata⚠
    LLVMSetNormalDest⚠
    LLVMSetOperand⚠
    LLVMSetOrdering⚠
    LLVMSetParamAlignment⚠
    LLVMSetParentCatchSwitch⚠
    LLVMSetPersonalityFn⚠
    LLVMSetSection⚠
    LLVMSetSourceFileName⚠
    LLVMSetSuccessor⚠
    LLVMSetTailCall⚠
    LLVMSetTarget⚠
    LLVMSetThreadLocal⚠
    LLVMSetThreadLocalMode⚠
    LLVMSetUnnamedAddress⚠
    LLVMSetUnwindDest⚠
    LLVMSetVisibility⚠
    LLVMSetVolatile⚠
    LLVMSetWeak⚠

    LLVMShutdown⚠

    LLVMSizeOf⚠




    */
