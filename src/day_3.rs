use std::collections::HashMap;

fn build_priorities() -> HashMap<char, usize> {
    ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>()
}

fn process_rucksack(s: &str) -> char {
    let (fir, sec) = s.split_at(s.len() / 2);
    fir.chars()
        .find_map(|c| sec.chars().find(|ch| *ch == c))
        .unwrap()
}

fn find_badge(sacks: [&str; 3]) -> char {
    let [fir, sec, thr] = sacks;
    fir.chars()
        .find(|c| sec.contains(*c) && thr.contains(*c))
        .unwrap()
}

pub fn day_3a(s: &str) -> String {
    let priorities = build_priorities();
    s.lines()
        .map(|line| {
            let sack = process_rucksack(line);
            let priority = priorities.get(&sack).unwrap();
            *priority as u32
        })
        .sum::<u32>()
        .to_string()
}

pub fn day_3b(s: &str) -> String {
    let priorities = build_priorities();
    s.lines()
        .array_chunks::<3>()
        .map(|group| {
            let badge = find_badge(group);
            let priority = priorities.get(&badge).unwrap();
            *priority
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_3a() {
        let s = day_3a(SHORT_INPUT);
        assert_eq!(s, "157");
    }
    #[test]
    fn test_day_3a_long() {
        let s = day_3a(LONG_INPUT);
        assert_eq!(s, "7428");
    }
    #[test]
    fn test_day_3b() {
        let s = day_3b(SHORT_INPUT);
        assert_eq!(s, "70");
    }
    #[test]
    fn test_day_3b_long() {
        let s = day_3b(LONG_INPUT);
        assert_eq!(s, "2650");
    }

    const SHORT_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    const LONG_INPUT: &str = "BccTFfTPTsffdDDqsq
lGGLQwFhDgWdqvhW
wbLNjGjlwLFrpSbllrHnHHRmmJVBmZJRRVcBTc
vvGLllBBLtllJnJFMZNjFcNG
bdhrhTgmhRrpLJMMNJgNffnf
qhRmTpzpzVzmTTbmVhWWWpVvDtsLVlwBVHvSsDCvvBDl
sLlhhthVRndBZzwBdV
PfmsPsqsGFqrSQpqBDDwpddDDM
mFjvSFrjPSvLJWvbRssN
gWnWQtMMDQbQvMDjjcwsvqcwsSSqZq
HzPJTtmtZJJsqfBJ
hCFPrNNPVNzFtQhdQMWRhhDd
sNhmsQZdNdsztNpHGCdjcBcwCvGvCv
brSlRfFFppHwfTpf
MbRlRpqMnqRpMhVtsQQQVWsM
HsdttdfHrHrwdhftMHMSDnJFwlQSgQZQZggZSW
LjCTqBCmmmTqgFQQFQjlnnDJ
GCTLBCmTBDBDPTTzqLCBpVcctcHbtbrcMMsdsdtcPtdb
QnJLdNLfLRQjpLlPflfQnlnswRvwsHVWVHWVVDZwWHcrwV
zFbgtqCBhtgBhsDZVDNHwvDv
zgCGCFCFGtMpJjlGTnlnLN
ZFhJZbvZVmFpFnJbnZFbLnbHPrjjQdBdCrMPQMQHrrrrVC
gTzlqmNlTflmtTRHdDPqMjHDMrjdQd
wcRSwglcNSwgWmNzwNlJZLpphbbnFhvLWhnphb
lRRrcbRfQmwwBLSLlvjjSvvFtS
WbghzhHWbsdHhVtGLVSSvLFJtSGC
WWbWzPHDqPfBqRZrmqnZ
frgLHMcHLrSZHPHcHrPNmMJtTCTCbJfhdwVbfJwwTbdd
DpDWRjQvqjDGFWhdVwQCnJgTnTbT
vvqggRsRpcssPrcsrN
MccbcZjmbbNSbjllbRqrhstVTMtqRtssTW
qJdwCLdCzQQzwHLQLTtsTtsTRVTVWWTJst
GdnpFHzwQCdpvpbccPbSbq
CbQhZpTbTgMMgptzrdGtGzlVNlrh
FWvmmRHSmjqnSjqnSjPqjnmNLGVGJlGWLlLrLWlVrGzNGW
FwvvBjmBnqmVvFnvnVPMDspDQTfZMCMTgDQTCB
VmrsHllHVLTdZtRclL
bjwMNQGCjGjjPwpRQnRLTtRQFvcdvt
MbWgjMDpNLMjNpwGpjbPWgfsHsqsHSrrHHJsgBSBzB
HFlwVhfLBZZLTLFHwqWCqWwQpWwSpqWC
PttsNjdtPtcmcDTnDdtjjDMpCCCCmSmqqGqmvMvbCqpC
jndRrTtPdgNPPnLrVhVLHhVJHffV
NJZVqHNNNJNqCdqZZVjtzCDMtzrtrFhhMFsPCS
TbwgwWRdQgfggQgvMPDSPszhSbSDszFs
QpRmWglwTQgGWTvGRvQRdwnjNNqqqBqVlcqVVNncVVHV
gqBFHLFDNCBClHgbLFbllLggVSTWJVWWpjvJpSSWJjBjZvjW
mczdshQcsGftdmcwcfhdzQVrhJZJTZhTVTDjpWpZvvWT
twGdRtcGnRgDqFRC
fsPjCjgRpwjPpsGgQwPfSZcdSZdWVHzSzcdzHRWz
MBLTTBMvbMmvJTLvTDvczNdZNHdHcJNzNqSZVG
rTlDlbBbrsjCCGnG
FsmBPtzHdmmvcvdWpcWCvM
RMJGjGGJJGLDRNJJnfGRRSvplTDccrTwSWClvCrwww
nnRqNGJLVLRRZNNZzBHMHVFVmzBFbhmb
MmTSpBMBCCMsbbrVvwwSLb
RffNRFRThtqHhHHqZNrLVvwQtsGvssdvvsGb
DThHghZfgqhFWRNHhhRWqMCnMzzznlPnBlgzMpnPCz
GhlMNMdpMZHHhhRLLjqSjjqvvmSh
PcTJDBDcBnBbTFcDwnPTQSLqLrqvWjwWLqRSvsRmWm
PDcDJbQTFBCBcncgQPcDgnHgVmppHVMddmZglzVHdMmt
BqFJqJGpBVnJqnjjmwvrNwVPHPrd
ZMbQmhWstZScWjdswwwHNdzHvj
ZZtQCtgZQWSbMMhmMSWQfGfGDBpRJnJqTFFJBgTR
tVcPGGqwgJPqtJtqZZwcZffsfrcNWBnSWWFrfhWBnz
dRMCMQLvNssBWFFR
pvjjQvmDMpHLdPPtgPPTssTHsq
WDBlFBbGdmBrqWjhWcqZCq
RPSRPtncngwgwzhCwghC
cssVsPpVncQMRsVQpRPnRcfJfTTmTdDBLbFlDblTDDDLJf
CZgCCzgzsCDZDzbbBclgvcjcnnjFBqgv
VLGThLWhWdThlWRHVTLTTjNjjdjrnBNBcrNcqmqqcc
VTRWLhpLWHRMPGRGRplVhSfttJpZzwstsbDtwbJzZt
fqhZRLhwZwVSLbCMCJdJCHQGZWdW
ztvpjtpcvgzppPvjvPlDQLnMlHDGWnQDLQWDdJ
PpsPzjPzgLrjzBgLggzrmrVSrTSfSqFVFmVhffFm
qMFDRLNRRJJphbhSgStpptbj
rZrPZZNCCCGrlfsCzBbgstBwtvbtvbvBth
CfVrdndzZGrfzrzdrGddPnNCFMRDJWRDDnRWDFqFDJJFTJJT
dMDbndBMQWnnbDstnMbMQQwspFgsFFjRrqCCFrrprqrgjc
SlLzZmHZSTVGSPVmSPlSmrpFvFjjpgrpgFcvrcFcWG
PHZZTmNVHSWJlHPNLSzPLLHwDBJtbDbnnnMffDbwtnMndQ
tHBzNCztLBRBtrjvSjfnjvvzfpjj
gQwWqlnqWqJgJnDplfflddjdsfdpFf
gcwwmnDccTPWWgJbZNNbZHZCRRTrLB
ZhjgtrJNfDNpqbhqQmqpmb
cFLcwGGHwcGdwFCQjnnnVcmqmjCp
HvvlzjWHjFWTggWStDrDZT
GPjjQtPQbjwWqrmnsjmnqn
SNGdfLLGdlldZSSRWznFsNqFRnqsWs
DdMpMfZMhfZMpDfbDQQbGQVgccgcVt
hHGGGTlddWGgpRdcvwDCDwzgLJzCwzwgwL
MFSbZSnnFJWwQBNMzw
tSjrSFSfqjqqttPnssqjssbhGmWGhcmvldcmhHRhGRhTRf
qdBjBTNndbnqnLmtZmZvvtLvHd
hDJCpfnGhJfDPzGzzCnGPmpZZVLZvHttZHQLHgZLHt
JDrfCDJhGMhFhPzMrCCnrGSwswswFsblTqRlRwjcjRsqBT
tJPRSZCSJJCnmvvvQMrpqLVwqLqMcCCM
hhGGGfsdfTGlfggjMLVVFsMVwMMbqbLF
hdTGGhGhdhfhhwlgNfdhZnZtHHRNtZHnnZmHHzmn
HlgRZglZDWZgfVbdznHddTvV
MhShhQMSFShFPPQplMPmwppwbzdtVbFtfvfbzVbvbTntzbVT
lMwpmJLLLNGwBrcZCjrGCGrD
qjjWRLjNjtGRRWTCghNwsgwcbwmC
HPMBpVGMPMvvVBPswmhTbCwgmsVThT
vDSMSvflBlHpvMMfSSpMFWZdtZWdGWGddqfFtRqQ
ZmnGQfnZgdmRGQGvgnnmHCbbchhpMLrpcbLpdLpzbz
NBFPBWFsSVNJlFqLDLwcwrPLzzCwhp
qlWJqqWFJJjNqJWsFVsVqqRCggtmvvjQZgQnZQmvmHRt
RdCsJbdsVJtRvdzBzrBcjzMTqT
wNSNnnHhnwhHfBHqHjBDJMJr
GnSPLwlFwwLWSwpNWLSNpVZtvPsVJsmRCmtgsvsRsv
GPMwMMPCBPwBGsCGGWqBsslQhVQFccphvhWpmcFhVLmV
rbNnHLLHfHfZFfQQQZml
DzSDNtrztbgCMMBLLCsBts
rTtrVbrrhbbGGhbbbbRqccdBcdBcvRvBSRRV
fDqQDqLFQgQgZFMlFNRgvCNvRRvHvRBHHR
mfPqmFsqqsbJJtGtbt
vNHvgsSGSFDHvspvtSGwJwJNJrllhwhcnwJlwm
dqWdWfQdqQVWwnfMnlcfwzMs
VqQQqBRTqsBQWLppZLpFSHZbvvDt
gdcldHQlQndnHMzCjvCFrzjSFtbLtv
NJGmZZJZGTDsWWJNmDVmsCrSvfPrvTtSPbFFCFFvCT
BDDsmJZVBJwBRsGGDmZNBGqMdnghhcwgbngpqqMgnpgc
VhhvVwmvmwTPCwPwmDRgDCsgWSnfRMSWSM
ptHZZtlZzqbZttHbzrFqnDSMShMgSRRMngDWgrGh
hBttqBhBzlFhtHvTJQJTcvBQJPPv
FjfzfGjsjBfQfMLBNg
VlrppVwrpVSdScwTjVnCCQLQNCMBhWgV
jvtSrSjjtSZpqtHPDPJqRzGR
ZSmbSDswfCDDHBFFvWHJ
jcjcVjltntQMltnVrdNnNjdQgFzWHqzBWGWQvzHwgqqJGz
nccLllwwhLCCSLTmmRTP
HBSnnJSfHvBfNWMNrvnMrZlblFbsbHwsPFVHTsHFls
VLmDLhRgRbZFRwjZ
zQDmCLLDDLGttGGgtLvvVSfqJBSVftSnrJnN
CgGhbgVMNgVVbjrrtcfTDTfvTscrTvSSss
qqFzFBzsPZHmddmsmvWSnHJnncDfWnDJvv
ZwdPqdqpdPlPwdRlsdBqBMjNQjGVgbQgQhGVCCtgbR
CGFTTMLGPgmPfGfCwTPSSFNQDLNQnscQNccWdQLqvqds
pbJlzhZRHtjrbrbjHHrplRWVNsNvddZNWDQVVcnQVscs
pzJtlJlBhJJHJjHhfPfTTFMDmMPBMSgm
gzCBPDDzgvLvgPLgNThPlVZccJTmrZNV
sndSnpsdMSnRMRpjShhNJZJhJjrVcTljTZ
SRRsdnGwRSpptnfMSSpdQtfdWqvbwFgbDJJzCzqqWqWbLWWz
BDnsPDlmmwcnCLLLwPtFTtTtFRqjRrrSVFqn
dfhhzMGzWJhpMWhHWggTHJVFVSSqbqqjdRRtvqqvFrtF
JMGfHffhQhQTNcTTBLCQmm
bfZBvvRRRzFfFFLRvFzZCcQlScchLlGNhSQGGVQh
mwJqTbsHmjbTNcNhQGGJGcVS
mPTgtsnPjwHHmmmbbRDgfpdgBpzvZD
lSnRStHtTZdjrHjnqJglbqgchhDCPCPc
swFBzvBNLpBBsvszvDJhCzgDDCgbbJQzDq
VswvBFmvpSrShmRRMn
vWBBSrWnZfCWVchwhbcjVN
RdHQQpRPJZLTtJgNcNGgbhGh
zpzDzTdqQRqRzzlRDsBBnBfMmCCSZDBS
ZtGSZVpPDtVbQjbwBDzbbL
WcFvTFnTMnnMcnhmQhmhBbBQzCQjVb
RRWsgvgnfqgpGVNg
WJTrJJCzLqCqBTWLsCCqzmPPQrjwHQQGpwGHHmRPVw
bMvnDFnFSbSlGgnpmGQVpRjV
DZSdFZdQZZclsscWqWLhcBhs
PzLlRRNjjRQzvPNQsvddlZfchhWWZJHSlhChDhHhcc
MMfngVwtpVMqVrwrMBgmtGqCHDWqJSDJWhHHSSSJJHHD
tBgnMTMMrgVmrBwMmGfnNjjsdLbbbTQvjNTNsQLz
ZZBZRmPmgpgZGLWLQWslSWmLQL
DnHJJjzqrJffrDnHzJjnMbQWSltttSbStvFQSstSvCLF
MzJDnfzHwHlljJJnqrMjfPGdBRpRBGBRhwVdGGRTBG
CnZCpMFNnFvvNdpHVrWghgtFVFlLRWhh
cBsSBsGcjGcJZDrggtgtVDVrgWmh
wTBJbcbzffJbZccjSbSjBfccNCNMPqqPPdvnHvTvnMTNdnvQ
vMgPmvQmWDMpGpjBbMMH
CcVJNcdNgdhtCVpjBBRppfRTGbph
sFsgNlcdFlJFFwFstNJcvzDWZqqWvqLqzLzmzDwQ
LLVLVsPPVVPCLLrjCNNNgmRdJNdCdfMJpB
DTZZHTWbwwpWbSWDBmJMSFFNmgRRBggf
ZvzDWqDnDwnZTpzZTzWvphPtqhQsltVtPhPhsQrPch
zDgWmDgrpCLmwgWTrjlJBQRJjbFGrcbQ
hSMvqvHtqsdVHlJDcclMBjRMQJ
sqvHsSstSdqhVVvZdqVHZDgwWpZZLLmfmmwfNfWCgmLg
SQWcTnWVWbZWWBcVPnZVbnrNrMFMdqFNqdMqqFhrDQvq
plGLlLGpJLhCGrRqGDDDrdzNGr
wgflHCCCJmpLjCLHtjjgLCtBZswnsTBPVPVsBcPTbZBBSh
vvlMQvvdjdGtVCTJlVJVfJ
FLrFqwwZgNrFWqZwgqrZBLWcBTtppztVbfRJztJbztfztT
WNmmFJwwrFFnNmDgmjdGQMdHMsPvPjsHDG
hGmZHdSRdMmhMZSHlvbTvRbRlVtCTlCR
znnfzgPPDpPfDcgnZTJvJNCZbJVCcNJV
nrLLfQznprrppgprWrnPzQzLSjGsZmHmhBdsqWhdhBMWhdqd
zhtNFSFwRFLCsNrNNBdl
BmQBPjDpBTDgHllgHc
npBjjpQpjGbMnmPpjPQWpwZhtbzJfhwvwtSwhhFFbh
PZcZbcPlbSprcQbbdCwWRSttgtgvWfjC
TGVLVHHmTVHGDTDnGDhgWjwvCjwwwRLLgBjWBR
CHTGsnHVVcJPPcNsNJ
tTqGSSGPGfVfTpqGTbbcVWJLdjtvdzjJthCjlhdlzQ
wwFBZMmZBmgnjzlCWBBjBLjv
rNMsDZnMMWSfDcDWPR
vLzbsczhLmmnlNvrNQHfWd
SMSFqMwjFFDVSZwVTMDjSQlQfNlRrQRWdQfRrWrqHN
CPTwGZDTFCPSjFTSPSFbPgpLscPczmcBLbfgpb
QnQnpFjsbFcSSvCMNvqVSrqq
WfzfTfzzPgHTfwfWtgRLMJDvjMmMVtvDJJCVtqmC
wfWRgPzdgRTWBBWHPBHHBRLTlcZdhjnbZQcGZGpsnphjshbG
pHzPTsBHzqqtQCZZshlWjf
DFFbnvJMDMljjtQjfCbQ
dgJFDGwgmGlMSggGdgdDDlvLNTzpBzLzzwpTBLzqPBczLT
jccNVNdwnclRwlbwlVjdcpJSpGpSllBHgGHZpJpppf
mDThTmsnDsSBpZfmmgBf
MrFrhCshqvWvnWzTWQtzVbRRQwQRNQjdwQ
tBnLJfnQtzRCffmNjSRjZjNZSRrP
dVdMVMvMghHzPhzZhHND
dGWGgplWGVMdMMzCTsbLtLTCLpnT
ftNfNDdSBdrMTdrjMM
cHgHGHzGgJhrPLqSrrJTqp
mVFnQnhQGHFznFhBBbSDRBlSVBBRfS
VqqPBPcPbQHgfrrpcSDR
tpMnsztnGnthhzTtGTGTzzWgJNDlMfSRlRDgRNRrDRHfrg
zWtTFWzTwphChnCzFhzWZGGvBvLqmVmbVQqjqjmBPqBLCP
ZZgZnhrmwmnmgmvrghPmgTGcTSGSMSldgcCQCSqW
LzFLDBfHzHCCqCFGcSlS
HBjRJDLpHpJsJVJqsnhP
PqrqmvmrwzznnPDpjVpDLfDtPGLt
sdRhRWFhShhFccZZsSsNbsNcjCtLpMVWCjptGCfMftBDLMtG
bVZcsdlhdNSbZRSshRcbbqlmnqHmmwzrJlzzgQlmvz
pnrcNGqmrGqnchGhqdWdTlldtQtlMsTq
DvSLgzLSMfbgggCLCwbSSLLtfsWTQTsWllssdltRQQtttQ
bCCzzPPgDPjPvwSzDbwpVNnpnjjrhpnVZGMhcp
jWbGtDdqCqZjdHwcwZMBVQmcvZ
PnTflPRRrlgLTTRlTzFPPQQBcNvHBncQpHMwHNBMwm
fFFRrFLJgRcJglgRzTzrLqWCjtqGGDsjCjdbGdqdhJ
GwbvGqMsDMbpMGzzgRzgpBLjhcch
WFTFNZTZSCcBggBFcrss
CJWWlsWlCtqGJmMGwJ
CGCVhprTrthCZTCNtVGtZDZNdlPPdPwmmvrcbmPmdQRvQWmw
LfzLzssfgHjLFjFLfjMfHsLHmPwdcQWQQlscwlPdQclbPvlw
LBFHjgMzqqjfJqLMzffHzqgHhhJDZSGVDVChCDDpDNNpNtDW
TbzVlmNTVVtnTSWNwDDrpGcwdp
fQQMFbhCfLgfQCsdDcHpsWpdSDsGrr
ghvPLQMfZhjjvPLhbQFQBZqJlTnnnVzRmtRzlmBl
JCLLLwVDwCQsNwwJHmfrMZpMfMMrfPQSMZpS
WlFlzFRnznthqWRGbMpVMbbMPtMjMj
vTnvzqllhdhqTwVBLcJHmmmC
tczhtcJJJbtclWrtJBWJBtJtpqPRSPfpBRgqRfPmpRqddSmM
HCvnsQLNCQwLnDsNHLwQfPSSpPSMfnPddRMmmGpp
wDQwjNwQNHjTHNFDCNmCFNWtWzhJbWbVrhtWccVFlrlV
sPRpCndBCGpCGHttSdvTbWvgdjST
wcmDwqcwmGDTNvjWtrbSrc
lwlVLVGqZGlLzVHHBBBCHBHRPCCz
wdmhffzzphrjqtzRbrrq
CRgGTGTFssZsllHNBlHsFJRjcrtjtPDPcbCtDrLcrjctrc
NHMRHTlFgGNwpQvMpwVvww
MPLJNPqmFWmDFjGS
nbsZtwbZlbZlGlFDDMpVlF
hvsbbbZtvfhhRZbZsfzMbMZbqcrqdrNrNqLgqhrLhJgddLNL
SnMLpRDGlZSZNlnMZpCwjLwzFrHBWCFWBBBr
TttvvtbtVcsJtRsvtQdzWJrBjCCHBWzrFFhCjC
VmvsQdgPbdgVTvgPMSDfPPZfDRRNDNMl
LjngLCNhDNFNhFDhcMqrqqZMcSZnHTMc
PPJwtGlfszGwWtzwQJBPGslJSqRTHZvgcRRZrMSTMMTtRTTc
llPPJWzQPmWmVNgDbLFCLb
NPFlLNBLprpdmmdPBmJnLrdjMVDjMSdqgggQTVDqWMdqVs
vZTbTZRwvvGRTRjWDqSqqQgDGsGq
vzZcfRRZbwbRHRtwZCChBmFNtJBPLBlBJJLpmFTP
pmvZmmTjQFfnvPPHHv
SczhzfbsLNhfccNFsWFRPrDnPDnVnW
BtBbSdtzLBwSLwBmTZpQMpZmZmfZ
PqPQZqtQQLDqrnqdjqdVwVbz
MGRGWMgJHGlRRHfSwfzCfCVVQzbwjrjn
JsmHmSJJmSMWMlTWQBFhLTvhDFZhBQDZ
sbgbbFGTTFNMbMNFWrjsrvWzHWPzPPpf
CVmhVqSqCZmJQhPpHzvZvtzWHjHP
CnmdCQCdnFTnNgRpwT
blZjhbZWVttjWjWLCLVVZCZQjMDQHsBsBQfMDQwjHDwBHH
NdcJdFcJqgpJpNnDQLMFsBnLSnnS
LJrJdJrzvdrrpcNdNcrVlZWbZzZVRhRtVlPttC
mhRtNNtrtBQQrtrBBmQlZwHHqHZSVHHGshSVDwhS
gpdPMTcsLscMccTpbLdHSfGfqwHZDHHqZqHZ
zbzvMpPLppLzLMjTBQRmssjBWRQjlmrN
VzzvggdvFdmffwmGpd
HNbnJTRTmCwwrRpR
LlTnWhLlhLJmLmtZtPcPcVFFPSZgZt
jHcZjHlHzLHHnSNSfL
pQWRrwPwrRWBWBPWBRrpdPmzhShsSFFNShLhnnvPtvSNNs
dzbmwVwbbBmGcqDbgllJCC
ndnvvzJDHvzHHHjnHjCCSDLgbSFwNFVbFVTL
mcQmQtpWTQGlmpTtMtqtpqTFsLcVSSscNCwLNLbwbbLNwV
pRZTQlhmtGWqqWnPHdnhjHrBjPPd
TtLpNHspTcLNNsLpthhsfmtjRSRlWWbzSwSRGwbWlWSSvlmS
qBJVnZZdJVZrZndbPbWwRzSMVGbbVS
QCdnBFBndBQDnrqrnqqNhpNNLpHthsThjGCTLL
phCgcdrFbPLpgrbFHqQqzzlbGWGqQbHW
SFTvTnVVMRnNTNfSHjHQMDlHwDWlQwDz
TZmTvsFNmvTtsggpdJLBBsCs
PBBWQjvsPsHVsNMcSzNDjcGggS
ZtrTfTrrrrdCqpdtLNnMLLqNcgMzgHLq
CZFmdTrJtbZrBvWHVVvHbPQW
djcrrBljMrTdCTcdCClClMlqRvtNqqSRwFbNbwvNBNpSzq
QhPmGJnPVGVHHNzSqpzFwztF
nhgPFmsnLPGLhPDJhGTcDjMfrMMjMZWfjfWj";
}
